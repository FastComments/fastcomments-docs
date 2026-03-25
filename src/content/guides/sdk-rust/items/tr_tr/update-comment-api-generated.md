## Parametreler

| Ad | Type | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| id | String | Evet |  |
| updatable_comment_params | models::UpdatableCommentParams | Evet |  |
| context_user_id | String | Hayır |  |
| do_spam_check | bool | Hayır |  |
| is_live | bool | Hayır |  |

## Yanıt

Döndürür: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Örnek

[inline-code-attrs-start title = 'update_comment Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let tenant_id: String = "acme-corp-tenant".into();
    let comment_id: String = "news/article/12345-6789".into();
    let context_user_id: String = "reader-42".into();

    let updatable: models::UpdatableCommentParams = models::UpdatableCommentParams {
        body: Some("Updated comment: I appreciate the clarification on this report.".into()),
        is_edited: Some(true),
        tags: Some(vec!["clarification".into(), "follow-up".into()]),
    };

    let params: UpdateCommentParams = UpdateCommentParams {
        tenant_id,
        id: comment_id,
        updatable_comment_params: updatable,
        context_user_id: Some(context_user_id),
        do_spam_check: Some(true),
        is_live: Some(false),
    };

    let response: FlagCommentPublic200Response = update_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---