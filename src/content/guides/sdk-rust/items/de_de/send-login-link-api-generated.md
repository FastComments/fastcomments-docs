## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| redirect_url | String | Nein |  |

## Antwort

Rückgabe: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Beispiel

[inline-code-attrs-start title = 'send_login_link Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(config: &configuration::Configuration) -> Result<(), Error> {
    let params = SendLoginLinkParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
        redirect_url: Some("https://acme.com/after-login".to_string()),
    };
    send_login_link(config, params).await?;
    Ok(())
}
[inline-code-end]