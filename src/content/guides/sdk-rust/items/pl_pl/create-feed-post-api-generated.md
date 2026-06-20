## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| create_feed_post_params | models::CreateFeedPostParams | Tak |  |
| broadcast_id | String | Nie |  |
| is_live | bool | Nie |  |
| do_spam_check | bool | Nie |  |
| skip_dup_check | bool | Nie |  |

## Odpowiedź

Zwraca: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_posts_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład create_feed_post'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<CreateFeedPostsResponse, Error> {
    let create_feed: models::CreateFeedPostParams = models::CreateFeedPostParams {
        title: "Acme Product Launch".to_string(),
        body: "Acme Corp today launched the next-generation WidgetPro, offering improved performance and battery life.".to_string(),
        ..Default::default()
    };
    let params: CreateFeedPostParams = CreateFeedPostParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_feed_post_params: create_feed,
        broadcast_id: Some("launch-broadcast-2026".to_string()),
        is_live: Some(true),
        do_spam_check: Some(true),
        skip_dup_check: Some(false),
    };
    let response: CreateFeedPostsResponse = create_feed_post(configuration, params).await?;
    Ok(response)
}
[inline-code-end]