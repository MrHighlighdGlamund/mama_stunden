    // Set some metadata
    //
    let mut doc = Handle::new();
    doc.info.author = Some(PdfString::new("Xiphoseer"));
    doc.info.creator = Some(PdfString::new("SIGNUM (c) 1986-93 F. Schmerbeck"));
    doc.info.producer = Some(PdfString::new("Signum! Document Toolbox"));
    doc.info.title = Some(PdfString::new("EMPTY.SDO"));
    let page = Page {
        media_box: Rectangle {
            ll: Point { x: 0, y: 0 },
            ur: Point { x: 592, y: 842 },
        },
        resources: Resources::default(),
        contents: Vec::new(),
    };
    doc.pages.push(page);

    // Write the PDF to the console
    std::fs::create_dir_all("/data/data/com.example.mutti/pdfs").unwrap();
    let mut pdf_writer: std::fs::File =
        std::fs::File::create("/data/data/com.example.mutti/pdfs/test.pdf").unwrap();

    doc.write(&mut pdf_writer).unwrap();
