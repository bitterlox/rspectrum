use mongodb::error::Error;
use mongodb::options::{FindOptions, InsertManyOptions, InsertOneOptions};
use mongodb::{options::ClientOptions, Client, Cursor};

pub struct Storage {
    handle: mongodb::Client,
    db: mongodb::Database,
}

impl Storage {
    pub async fn new() -> Storage {
        let client_options =
            ClientOptions::parse("mongodb://spectrum:ubq4lyfe@localhost:27017").await;

        match client_options {
            Ok(opts) => {
                let r = Client::with_options(opts);

                if let Ok(client) = r {
                    Storage {
                        handle: client.clone(),
                        db: client.clone().database("spectrumdb"),
                    }
                } else {
                    panic!("Couldn't create mongo client")
                }
            }
            Err(error) => {
                println!("{:?}", error);
                panic!()
            }
        }
    }

    pub async fn set_key(&self) {
        let res = self
            .db
            .collection("test")
            .insert_many(
                vec![doc! {"field1": 69}, doc! {"field1": 79}],
                InsertManyOptions::default(),
            )
            .await;
        if let Ok(val) = res {
            println!("{:?}", val);
        }
    }

    pub async fn get_key(&self) -> Result<Cursor, Error> {
        self.db
            .collection("test")
            .find(doc! {}, FindOptions::default())
            .await
    }
}
