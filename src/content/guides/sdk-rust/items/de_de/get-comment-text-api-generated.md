## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| edit_key | String | Nein |  |
| sso | String | Nein |  |

## Antwort

Rückgabe: [`PublicApiGetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_api_get_comment_text_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_comment_text Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        edit_key: Some("edit-key-abc".to_string()),
        sso: Some("sso-token-xyz".to_string()),
    };
    let _response = get_comment_text(&configuration, params).await?;
    Ok(())
}
[inline-code-end]