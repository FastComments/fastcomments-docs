## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| comment_id | String | Oui |  |
| broadcast_id | String | Oui |  |
| sso | String | Non |  |

## Réponse

Renvoie : [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/change_comment_pin_status_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de un_pin_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<ChangeCommentPinStatusResponse, Error> {
    let params: UnPinCommentParams = UnPinCommentParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("cmt-8f3b2a1e"),
        broadcast_id: String::from("news/2024/product-launch"),
        sso: Some(String::from("sso-user-abcdef123456")),
    };
    let response: ChangeCommentPinStatusResponse = un_pin_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---