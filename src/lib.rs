pub mod ghost {
    use chrono::{DateTime, Utc};
    use serde::Deserialize;
    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    #[derive(Debug, Deserialize)]
    pub struct Post {
        pub title: String,
        pub slug: String,
        pub mobiledoc: String,
        pub comment_id: String,
        pub feature_image: Option<String>,
        pub created_at: DateTime<Utc>,
        pub published_at: Option<String>,
        pub r#type: String,
    }

    #[derive(Debug, Deserialize)]
    struct ExportData {
        posts: Vec<Post>,
    }

    #[derive(Debug, Deserialize)]
    struct ExportDb {
        data: ExportData,
    }

    #[derive(Debug, Deserialize)]
    struct ExportFile {
        db: Vec<ExportDb>,
    }

    pub fn read_posts_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Post>, Box<dyn Error>> {
        // Open the file in read-only mode with buffer.
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of ExportFile.
        let ExportFile { mut db } = serde_json::from_reader(reader)?;

        if let Some(ExportDb { data }) = db.pop() {
            Ok(data.posts)
        } else {
            Ok(vec![])
        }
    }
}
