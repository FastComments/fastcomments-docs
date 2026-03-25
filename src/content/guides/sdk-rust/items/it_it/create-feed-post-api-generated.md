## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| create_feed_post_params | models::CreateFeedPostParams | Sì |  |
| broadcast_id | String | No |  |
| is_live | bool | No |  |
| do_spam_check | bool | No |  |
| skip_dup_check | bool | No |  |

## Risposta

Restituisce: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_200_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di create_feed_post'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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