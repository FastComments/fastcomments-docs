---
## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| feed_post | models::FeedPost | Ja |  |

## Svar

Returnerer: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Eksempel

[inline-code-attrs-start title = 'update_feed_post Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let _response: ApiEmptyResponse = update_feed_post(&configuration, UpdateFeedPostParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("news/quarterly-product-update"),
        feed_post: models::FeedPost {
            title: Some(String::from("Quarterly Product Update")),
            content: Some(String::from("We shipped new features and improvements across the platform.")),
            tags: Some(vec![String::from("product"), String::from("release")]),
            media: Some(vec![models::FeedPostMediaItem {
                asset: Some(models::FeedPostMediaItemAsset {
                    url: String::from("https://cdn.acme.com/images/update.jpg"),
                    mime_type: Some(String::from("image/jpeg")),
                }),
                caption: Some(String::from("New dashboard view")),
                order: Some(0),
            }]),
            links: Some(vec![models::FeedPostLink {
                url: String::from("https://acme.com/blog/product-update"),
                title: Some(String::from("Read the full post")),
            }]),
        },
    }).await?;
    Ok(())
}
[inline-code-end]

---