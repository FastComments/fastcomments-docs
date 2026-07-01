## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenant_id | String | Oui |  |
| comment_id | String | Oui |  |
| broadcast_id | String | Non |  |
| sso | String | Non |  |

## Réponse

Renvoie : [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/post_remove_comment_api_response.rs)

## Exemple

[inline-code-attrs-start title = 'post_remove_comment Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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