use std::{collections::BTreeMap, fs::File, io::BufWriter};

use pdf_canvas::graphicsstate::Color;
use pdf_canvas::{BuiltinFont, Pdf};

use crate::database::Entry;

pub struct PrintData {
    pub name: String,
    pub month: String,
    pub year: i32,
    pub entries: Vec<Entry>,
    pub overtime_balance_before: f32,
    pub overtime_balance_after: f32,
    pub total: f32,
}

pub fn create_pdf(print_data: PrintData) -> String {
    let file_name = format!(
        "/storage/emulated/0/Download/Abrechnung_{}Katier_{}_{}.pdf",
        print_data.name, print_data.month, print_data.year
    );
    if std::path::Path::new(&file_name).exists() {
        std::fs::remove_file(&file_name).unwrap();
    }

    let mut document = Pdf::create(&file_name).unwrap();
    document.set_title("Abrechnung");
    document
        .render_page(210.0 * 2.83465, 297.0 * 2.83465, |c| {
            let helvetica = BuiltinFont::Helvetica;
            let mut line_pos = 297.0 * 2.83465 - 25.0 * 2.83465;
            c.center_text(
                105.0 * 2.83465,
                line_pos,
                BuiltinFont::Times_Bold,
                8.0 * 2.83465,
                format!("Abrechnung {} Katier", print_data.name).as_str(),
            );
            line_pos -= 12.0 * 2.83465;
            c.center_text(
                105.0 * 2.83465,
                line_pos,
                BuiltinFont::Times_Bold,
                8.0 * 2.83465,
                format!("{} {}", print_data.month, print_data.year).as_str(),
            );
            line_pos -= 10.0 * 2.83465;
            c.center_text(
                105.0 * 2.83465,
                line_pos,
                BuiltinFont::Times_Bold,
                6.5 * 2.83465,
                format!(
                    "Stunden Balance Vormonat: {} Std.",
                    print_data.overtime_balance_before
                )
                .as_str(),
            );
            line_pos -= 12.0 * 2.83465;

            for (i, line) in print_data.entries.iter().enumerate() {
                c.left_text(
                    25.4 * 2.83465,
                    line_pos,
                    BuiltinFont::Times_Bold,
                    5.0 * 2.83465,
                    line.date.as_str(),
                );
                c.center_text(
                    105.0 * 2.83465,
                    line_pos,
                    BuiltinFont::Times_Bold,
                    5.0 * 2.83465,
                    line.text.as_str(),
                );
                c.right_text(
                    210.0 * 2.83465 - 25.4 * 2.83465,
                    line_pos,
                    BuiltinFont::Times_Bold,
                    5.0 * 2.83465,
                    format!("{} Std.", line.hours).as_str(),
                );
                line_pos -= 6.5 * 2.83465;
            }

            line_pos -= 6.0 * 2.83465;
            c.left_text(
                25.4 * 2.83465,
                25.4 * 2.83465,
                BuiltinFont::Times_Bold,
                6.5 * 2.83465,
                format!(
                    "Stunden Balance: {} Std.",
                    print_data.overtime_balance_after
                )
                .as_str(),
            );
            c.right_text(
                210.0 * 2.83465 - 25.4 * 2.83465,
                25.4 * 2.83465,
                BuiltinFont::Times_Bold,
                6.5 * 2.83465,
                format!("Gesamt: {} Std.", print_data.total).as_str(),
            );

            Ok(())
        })
        .unwrap();
    document.finish().unwrap();
    file_name
}
