## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenant_id | String | Tak |  |
| comment_id | String | Tak |  |
| broadcast_id | String | Nie |  |
| sso | String | Nie |  |

## Odpowiedź

Zwraca: [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/post_remove_comment_api_response.rs)

## Przykład

[inline-code-attrs-start title = 'post_remove_comment Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---