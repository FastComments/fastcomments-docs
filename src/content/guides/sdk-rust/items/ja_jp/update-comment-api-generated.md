## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| id | String | はい |  |
| updatable_comment_params | models::UpdatableCommentParams | はい |  |
| context_user_id | String | いいえ |  |
| do_spam_check | bool | いいえ |  |
| is_live | bool | いいえ |  |

## レスポンス

戻り値: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 例

[inline-code-attrs-start title = 'update_comment の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: UpdateCommentParams = UpdateCommentParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "news/article-2026/comments/12345".to_string(),
    updatable_comment_params: models::UpdatableCommentParams {
        content: "Thanks for the update — I corrected the typo and clarified the timeline.".to_string(),
        ..Default::default()
    },
    context_user_id: Some("editor-42".to_string()),
    do_spam_check: Some(true),
    is_live: Some(true),
};

let response: ApiEmptyResponse = update_comment(&configuration, params).await?;
[inline-code-end]