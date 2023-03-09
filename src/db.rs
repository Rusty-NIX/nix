use mongodb::{bson::doc, sync::Client};

pub fn db() -> mongodb::error::Result<()> {
    // Get a handle to the cluster
    let client = Client::with_uri_str(
        "mongodb+srv://admin:aep564301@cluster0.i38wpgb.mongodb.net/?retryWrites=true&w=majority",
    )?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)?;
    println!("Connected successfully.");

    // List the names of the databases in that cluster
    for db_name in client.list_database_names(None, None)? {
        println!("{}", db_name);
    }
    Ok(())
}