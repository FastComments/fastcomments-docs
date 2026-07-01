## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| broadcast_id | String | Nein |  |
| sso | String | Nein |  |

## Antwort

Rückgabe: [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/post_remove_comment_api_response.rs)

## Beispiel

[inline-code-attrs-start title = 'post_remove_comment Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn remove_comment_example(config: &configuration::Configuration) -> Result<(), Error> {
    let params = PostRemoveCommentParams {
        tenant_id: "acme-corp".into(),
        comment_id: "news/article/42".into(),
        broadcast_id: Some("live-event-99".into()),
        sso: Some("sso-user-abc".into()),
    };
    let _response = post_remove_comment(config, params).await?;
    Ok(())
}
[inline-code-end]