## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenant_id | String | Ja |  |
| create_feed_post_params | models::CreateFeedPostParams | Ja |  |
| broadcast_id | String | Nee |  |
| is_live | bool | Nee |  |
| do_spam_check | bool | Nee |  |
| skip_dup_check | bool | Nee |  |

## Respons

Retourneert: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_posts_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'create_feed_post Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = CreateFeedPostParams {
        tenant_id: "acme-corp-tenant".into(),
        create_feed_post_params: models::CreateFeedPostParams {
            text: "Launching new features".into(),
            media: vec![],
        },
        broadcast_id: Some("broadcast-2023-09".into()),
        is_live: Some(true),
        do_spam_check: Some(true),
        skip_dup_check: Some(false),
    };
    let _response = create_feed_post(configuration, params).await?;
    Ok(())
}
[inline-code-end]