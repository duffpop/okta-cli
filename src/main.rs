mod okta;
use ::okta::{types::User, users::Users, ClientError};
use okta::client::OktaClient;

#[tokio::main]
async fn main() {
    let client = OktaClient::new();
    match list_users(&client).await {
        Ok(users) => {
            for user in users {
                if let Some(profile) = &user.profile {
                    println!("User: {}", profile.login);
                } else {
                    println!("User profile is missing");
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

// async function to list all users
async fn list_users(client: &OktaClient) -> Result<Vec<User>, ClientError> {
    let users = Users {
        client: client.get_client().clone(),
    };

    // Using list_all for simplicity - returns all users without pagination
    let response = users
        .list_all(
            "",     // q - no search query
            "",     // filter - no filter
            "",     // search - no search
            "",     // sort_by
            "desc", // sort_order
        )
        .await?;

    Ok(response.body)
}
