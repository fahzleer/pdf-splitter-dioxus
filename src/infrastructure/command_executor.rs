use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::Arc;
use printpdf::{Image, ImageTransform, Mm, PdfDocument};
use crate::domain::commands::PdfCommand;
use crate::domain::errors::SplitError;
use crate::domain::ports::{CommandExecutor, FileSystem, PdfReader, PdfWriter};

pub struct DefaultCommandExecutor {
    reader: Arc<dyn PdfReader>,
    writer: Arc<dyn PdfWriter>,
    fs: Arc<dyn FileSystem>,
    doc_cache: Mutex<HashMap<PathBuf, lopdf::Document>>,
}

impl DefaultCommandExecutor {
    pub fn new(
        reader: Arc<dyn PdfReader>,
        writer: Arc<dyn PdfWriter>,
        fs: Arc<dyn FileSystem>,
    ) -> Self {
        DefaultCommandExecutor {
            reader,
            writer,
            fs,
            doc_cache: Mutex::new(HashMap::new()),
        }
    }

    fn execute_create_pdf_from_images(
        &self,
        output_path: &PathBuf,
        images: &[crate::domain::models::ImageSource],
        page_size: &crate::domain::models::PageSizeOption,
    ) -> Result<(), SplitError> {
        if images.is_empty() {
            return Err(SplitError::WriteError {
                msg: "no images provided".into(),
            });
        }

        let mut doc_ref: Option<printpdf::PdfDocumentReference> = None;

        for img_src in images.iter() {
            let img = image::open(&img_src.path)
                .map_err(|e| SplitError::WriteError { msg: e.to_string() })?;
            let printpdf_image = Image::from_dynamic_image(&img);

            let img_width_px = img_src.width as f32;
            let img_height_px = img_src.height as f32;

            let (page_w_mm, page_h_mm) = match page_size {
                crate::domain::models::PageSizeOption::FitImage => {
                    let w = img_src.width as f32 * 0.264583;
                    let h = img_src.height as f32 * 0.264583;
                    (w, h)
                }
                crate::domain::models::PageSizeOption::A4 => (210.0, 297.0),
            };

            let page_w = Mm(page_w_mm);
            let page_h = Mm(page_h_mm);

            let scale_x = page_w_mm / img_width_px;
            let scale_y = page_h_mm / img_height_px;
            let scale = scale_x.min(scale_y);

            let layer = if let Some(ref d) = doc_ref {
                let (p, l) = d.add_page(page_w, page_h, "Layer 1");
                d.get_page(p).get_layer(l)
            } else {
                let (d, p, l) = PdfDocument::new("Images to PDF", page_w, page_h, "Layer 1");
                let layer = d.get_page(p).get_layer(l);
                doc_ref = Some(d);
                layer
            };

            printpdf_image.add_to_layer(
                layer,
                ImageTransform {
                    translate_x: Some(Mm(0.0)),
                    translate_y: Some(Mm(0.0)),
                    scale_x: Some(scale),
                    scale_y: Some(scale),
                    ..Default::default()
                },
            );
        }

        if let Some(doc) = doc_ref {
            let file = std::fs::File::create(output_path).map_err(|e| {
                SplitError::WriteError { msg: e.to_string() }
            })?;
            let mut buf = std::io::BufWriter::new(file);
            doc.save(&mut buf)
                .map_err(|e| SplitError::WriteError { msg: e.to_string() })?;
        }

        Ok(())
    }
}

impl CommandExecutor for DefaultCommandExecutor {
    fn execute(&self, command: &PdfCommand) -> Result<(), SplitError> {
        match command {
            PdfCommand::EnsureDir { path } => self.fs.ensure_dir(path),
            PdfCommand::ExtractAndSave {
                doc_path,
                range,
                output_path,
                overwrite,
            } => {
                if self.fs.file_exists(output_path) && !overwrite {
                    return Err(SplitError::WriteError {
                        msg: format!("file already exists: {}", output_path.display()),
                    });
                }

                let mut cache = self.doc_cache.lock().unwrap();
                let doc_inner = cache.entry(doc_path.clone()).or_insert_with(|| {
                    self.reader.load(doc_path).unwrap().inner
                });
                let doc = crate::domain::ports::PdfDocument { inner: doc_inner.clone() };
                drop(cache);

                let data = self.writer.extract_pages(&doc, range)?;
                self.writer.save(&data, output_path)
            }
            PdfCommand::CreatePdfFromImages {
                output_path,
                images,
                page_size,
            } => self.execute_create_pdf_from_images(output_path, images, page_size),
            PdfCommand::Log { .. } => Ok(()),
        }
    }
}
