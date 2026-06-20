## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |
| redirect_url | String | Non |  |

## Réponse

Renvoie : [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de send_login_link'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn send_link_example() -> Result<(), Error> {
    let params: SendLoginLinkParams = SendLoginLinkParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-9876".to_string(),
        redirect_url: Some("https://acme.example.com/welcome".to_string()),
    };
    let response: ApiEmptyResponse = send_login_link(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---