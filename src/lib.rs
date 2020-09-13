pub mod ghost {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer};
    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    #[derive(Debug, Deserialize)]
    pub struct Card {
        #[serde(default)]
        pub markdown: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct MobileDoc {
        pub cards: Vec<(String, Card)>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Post {
        pub title: String,
        pub slug: String,
        #[serde(deserialize_with = "mobile_doc")]
        pub mobiledoc: MobileDoc,
        pub comment_id: String,
        pub feature_image: Option<String>,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
        pub published_at: Option<String>,
        pub r#type: String,
    }

    fn mobile_doc<'de, D>(deserializer: D) -> Result<MobileDoc, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        serde_json::from_str(&s).map_err(serde::de::Error::custom)
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

pub mod zola {
    use crate::ghost;
    use chrono::{DateTime, Utc};
    use std::fmt;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use std::str;

    pub struct Page {
        /// The markdown content of the post
        content: String,
        /// The date of the page
        date: DateTime<Utc>,
        /// Whether the page is published yet or not
        draft: bool,
        /// Slug for the url of the page
        slug: String,
        /// Title of the page
        title: String,
        /// The last updated date of the post
        updated: DateTime<Utc>,
        // TODO: taxonomies: Vec<String>,
    }

    impl From<ghost::Post> for Page {
        fn from(
            ghost::Post {
                created_at,
                mobiledoc,
                slug,
                published_at,
                title,
                updated_at,
                ..
            }: ghost::Post,
        ) -> Self {
            let content = mobiledoc
                .cards
                .iter()
                .fold(String::new(), |mut acc, (_, c)| {
                    acc.push_str(&c.markdown);
                    acc
                });
            Self {
                content,
                date: created_at,
                draft: published_at.is_none(),
                slug,
                title,
                updated: updated_at,
            }
        }
    }

    impl fmt::Display for Page {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Frontmatter
            writeln!(f, "+++")?;
            writeln!(f, "title = \"{}\"", str::replace(&self.title, "\"", "\\\""))?;
            writeln!(f, "slug = \"{}\"", self.slug)?;
            writeln!(f, "date = {}", self.date.format("%Y-%m-%d"))?;
            writeln!(f, "updated = {}", self.updated.format("%Y-%m-%d"))?;
            writeln!(f, "draft = {}", self.draft)?;
            writeln!(f, "+++")?;
            // Body
            writeln!(f, "{}", self.content)
        }
    }

    impl Page {
        pub fn write_to_file(&self) {
            let filename = format!(
                "../myblog/content/{}-{}.md",
                self.date.format("%Y-%m-%d"),
                self.slug
            );
            let path = Path::new(&filename);
            let display = path.display();

            // Open a file in write-only mode, returns `io::Result<File>`
            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", display, why),
                Ok(file) => file,
            };

            // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
            match file.write_all(&format!("{}", self).as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why),
                Ok(_) => println!("successfully wrote to {}", display),
            }
        }
    }
}
