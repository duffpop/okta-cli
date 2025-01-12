mod okta;
// use crate::okta::OktaClient;
use ::okta::types::User;
use ::okta::users::Users;
use ::okta::ClientError;
use okta::client::OktaClient;
use skim::prelude::*;

use std::borrow::Cow;
use std::sync::Arc;

pub struct MenuItem {
    text: String,
    preview: String,
}

impl SkimItem for MenuItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.text)
    }

    fn preview(&self, _: PreviewContext) -> ItemPreview {
        ItemPreview::Text(self.preview.clone())
    }
}

pub struct OktaMenu {
    client: OktaClient,
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

impl OktaMenu {
    pub fn new(client: OktaClient) -> Self {
        Self { client }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let main_options = vec![
            MenuItem {
                text: "Users".to_string(),
                preview: "View and manage Okta users".to_string(),
            },
            MenuItem {
                text: "Groups".to_string(),
                preview: "View and manage Okta groups".to_string(),
            },
            MenuItem {
                text: "Apps".to_string(),
                preview: "View and manage Okta applications".to_string(),
            },
        ];

        if let Some(selection) = self.show_menu(main_options, "Select an option:".to_string())? {
            match selection.as_str() {
                "Users" => self.show_users_menu().await?,
                "Groups" => self.show_groups_menu().await?,
                "Apps" => self.show_apps_menu().await?,
                _ => println!("Invalid selection"),
            }
        }

        Ok(())
    }

    fn show_menu(
        &self,
        items: Vec<MenuItem>,
        prompt: String,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let options = SkimOptionsBuilder::default()
            .height("100%".to_string())
            .prompt(prompt)
            .preview(Some("".to_string()))
            .build()?;

        let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

        for item in items {
            let _ = tx_item.send(Arc::new(item));
        }
        drop(tx_item);

        let selected = Skim::run_with(&options, Some(rx_item))
            .map(|out| {
                out.selected_items
                    .first()
                    .map(|item| item.text().to_string())
            })
            .unwrap_or(None);

        Ok(selected)
    }

    async fn show_users_menu(&self) -> Result<(), Box<dyn std::error::Error>> {
        let users = list_users(&self.client).await?;
        let items: Vec<MenuItem> = users
            .into_iter()
            .map(|user| MenuItem {
                text: user
                    .profile
                    .as_ref()
                    .and_then(|p| Some(p.login.clone()))
                    .unwrap_or_default(),
                preview: format!("{:#?}", user),
            })
            .collect();

        self.show_menu(items, "Select a user:".to_string())?;
        Ok(())
    }

    async fn show_groups_menu(&self) -> Result<(), Box<dyn std::error::Error>> {
        let groups = self
            .client
            .get_client()
            .groups()
            .list("", "", "", 500, "desc")
            .await?;
        let items: Vec<MenuItem> = groups
            .body
            .into_iter()
            .map(|group| {
                let preview = format!("{:#?}", group);
                MenuItem {
                    text: group.profile.and_then(|p| Some(p.name)).unwrap_or_default(),
                    preview,
                }
            })
            .collect();

        self.show_menu(items, "Select a group:".to_string())?;
        Ok(())
    }

    async fn show_apps_menu(&self) -> Result<(), Box<dyn std::error::Error>> {
        let apps = self
            .client
            .get_client()
            .applications()
            .list("", "", 500, "", "", true)
            .await?;

        let items: Vec<MenuItem> = apps
            .body
            .into_iter()
            .map(|app| {
                let preview = format!("{:#?}", app);
                MenuItem {
                    text: app.name,
                    preview,
                }
            })
            .collect();

        self.show_menu(items, "Select an application:".to_string())?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let client = OktaClient::new();
    let menu = OktaMenu::new(client);

    if let Err(e) = menu.run().await {
        eprintln!("Error: {}", e);
    }
}
