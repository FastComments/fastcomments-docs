## Parametreler

| Ad | Tip | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| id | String | Evet |  |
| send_email | String | Hayır |  |

## Yanıt

Döner: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Örnek

[inline-code-attrs-start title = 'delete_moderator Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = DeleteModeratorParams {
        tenant_id: "acme-corp".to_string(),
        id: "moderator-123".to_string(),
        send_email: Some("admin@acme.com".to_string()),
    };
    let _ = delete_moderator(&configuration, params).await?;
    Ok(())
}
[inline-code-end]