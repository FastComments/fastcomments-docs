## Parameters

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| broadcast_id | String | Ja |  |
| sso | String | Nein |  |

## Antwort

Rückgabe: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/change_comment_pin_status_response.rs)

## Beispiel

[inline-code-attrs-start title = 'un_pin_comment Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn unpin_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = UnPinCommentParams {
        tenant_id: "acme-corp".to_string(),
        comment_id: "comment-12345".to_string(),
        broadcast_id: "news/article-6789".to_string(),
        sso: Some("sso-token-xyz".to_string()),
    };
    let _response: ChangeCommentPinStatusResponse = un_pin_comment(configuration, params).await?;
    Ok(())
}
[inline-code-end]