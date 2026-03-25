## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| create_feed_post_params | models::CreateFeedPostParams | Ja |  |
| broadcast_id | String | Nee |  |
| is_live | bool | Nee |  |
| do_spam_check | bool | Nee |  |
| skip_dup_check | bool | Nee |  |

## Respons

Retourneert: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'create_feed_post Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn create_post_example() -> Result<CreateFeedPost200Response, Error> {
    let params = CreateFeedPostParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_feed_post_params: models::CreateFeedPostParams {
            title: Some("Downtown Datacenter Outage".to_string()),
            body: Some("Investigating a partial outage affecting login and API endpoints.".to_string()),
            author_id: Some("ops-team".to_string()),
            ..Default::default()
        },
        broadcast_id: Some("status-broadcast-2026-03-25".to_string()),
        is_live: Some(true),
        do_spam_check: Some(true),
        skip_dup_check: Some(false),
    };
    let response: CreateFeedPost200Response = create_feed_post(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---