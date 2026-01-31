## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_question_result_body | models::CreateQuestionResultBody | Yes |  |

## Response

Returns: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_question_result_200_response.rs)

## Example

[inline-code-attrs-start title = 'create_question_result Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn create_question_result_example() -> Result<CreateQuestionResult200Response, Error> {
    let params: CreateQuestionResultParams = CreateQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_question_result_body: models::CreateQuestionResultBody {
            question_id: "article-readability-1".to_string(),
            comment_id: Some("cmt-000112".to_string()),
            user_id: Some("reader_2048".to_string()),
            answers: Some(vec!["clear".to_string(), "engaging".to_string()]),
            score: Some(4),
            tags: Some(vec!["news".to_string(), "opinion".to_string()]),
            metadata: Some(std::collections::HashMap::from([
                ("article_path".to_string(), "news/article/2026/01/13".to_string())
            ])),
        },
    };
    let response: CreateQuestionResult200Response = create_question_result(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
