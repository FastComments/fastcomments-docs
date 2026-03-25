## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| updatable_comment_params | models::UpdatableCommentParams | Yes |  |
| context_user_id | String | No |  |
| do_spam_check | bool | No |  |
| is_live | bool | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'update_comment Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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
