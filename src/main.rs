mod okta;
use ::okta::{types::User, users::Users, ClientError};
use okta::client::OktaClient;
use skim::prelude::*;
use std::borrow::Cow;

struct UserItem {
    user: User,
}

impl SkimItem for UserItem {
    fn text(&self) -> Cow<str> {
        Cow::Owned(
            self.user
                .profile
                .as_ref()
                .map(|p| p.login.clone())
                .unwrap_or_else(|| "No login".to_string()),
        )
    }

    fn preview(&self, _: PreviewContext) -> ItemPreview {
        let preview_text = if let Some(profile) = &self.user.profile {
            format!("{:#?}", profile)
        } else {
            "No profile information available".to_string()
        };

        ItemPreview::Text(preview_text)
    }
}

#[tokio::main]
async fn main() {
    let client = OktaClient::new();
    match list_users(&client).await {
        Ok(users) => {
            let options = SkimOptionsBuilder::default()
                .height("80%".to_string())
                .multi(true)
                .preview(Some("".to_string()))
                .build()
                .unwrap();

            let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

            for user in users {
                let _ = tx_item.send(Arc::new(UserItem { user }));
            }
            drop(tx_item);

            if let Some(output) = Skim::run_with(&options, Some(rx_item)) {
                for item in output.selected_items.iter() {
                    println!("{}", item.output());
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

async fn list_users(client: &OktaClient) -> Result<Vec<User>, ClientError> {
    let users = Users {
        client: client.get_client().clone(),
    };

    users
        .list_all("", "", "", "", "desc")
        .await
        .map(|response| response.body)
}
