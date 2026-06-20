## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| id | String | Sí |  |
| from_name | String | Sí |  |

## Respuesta

Devuelve: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de send_invite'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_send_invite() -> Result<ApiEmptyResponse, Error> {
    let params: SendInviteParams = SendInviteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-2026-06-19".to_string(),
        from_name: "Acme News Team".to_string(),
        subject: Some("Invitation to comment".to_string()),
        message: Some("We value your feedback on this article — join the conversation.".to_string()),
        ..Default::default()
    };

    let response: ApiEmptyResponse = send_invite(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]