## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_question_result_body | models::UpdateQuestionResultBody | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'update_question_result Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<FlagCommentPublic200Response, Error> {
    let params: UpdateQuestionResultParams = UpdateQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/comment-9876".to_string(),
        update_question_result_body: models::UpdateQuestionResultBody {
            question_id: "q-2026-01".to_string(),
            respondent: Some("moderator@acme.news".to_string()),
            outcome: Some("review_required".to_string()),
            correct: Some(false),
            score: Some(0.4),
        },
    };
    let response: FlagCommentPublic200Response = update_question_result(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
