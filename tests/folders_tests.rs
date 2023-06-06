#[cfg(test)]
mod folders_tests {
    use std::env;
    use serde::de::Unexpected::Str;
    use box_rust_sdk::authorization::DeveloperTokenAuthorizaton;
    use box_rust_sdk::box_api_client::BoxApiClient;
    use box_rust_sdk::managers::folders;
    use box_rust_sdk::managers::folders::CreateFolderRequest;
    use box_rust_sdk::models::ItemType::{File, Folder, WebLink};

    #[tokio::test]
    async fn folder_items_works() {
        let items = folders::items(&prepare_client(), &String::from("0")).await;

        assert_eq!(items.entries.is_some(), true, "Missing items");
        let item_type = items.entries.unwrap().get(0).unwrap().item_type;
        assert_eq!([Folder, File, WebLink].contains(&item_type), true, "Unknown item type")
    }

    #[tokio::test]
    async fn folder_create_delete_get_works() {
        let body = CreateFolderRequest::new(String::from("Test"), String::from("0"));

        let api = &prepare_client();
        let folder = folders::create(api, &body).await;
        let folder = folders::get(api, &folder.id).await;
        assert_eq!(folder.is_some(), true, "Folder was not created");

        let folder_id = &folder.unwrap().id;
        folders::delete(api, folder_id).await;
        let folder = folders::get(api, folder_id).await;
        assert_eq!(folder.is_none(), true, "Folder was not removed");
    }

    fn prepare_client() -> BoxApiClient {
        dotenv::dotenv().expect("Failed to read .env file");
        let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");
        let authorizaton = DeveloperTokenAuthorizaton::new(developer_token);
        BoxApiClient::new(authorizaton)
    }
}
