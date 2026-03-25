## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| updatable_comment_params | models::UpdatableCommentParams | Da |  |
| context_user_id | String | Ne |  |
| do_spam_check | bool | Ne |  |
| is_live | bool | Ne |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer update_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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