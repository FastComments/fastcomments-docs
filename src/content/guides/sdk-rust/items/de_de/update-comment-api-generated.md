## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| updatable_comment_params | models::UpdatableCommentParams | Ja |  |
| context_user_id | String | Nein |  |
| do_spam_check | bool | Nein |  |
| is_live | bool | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'update_comment Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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