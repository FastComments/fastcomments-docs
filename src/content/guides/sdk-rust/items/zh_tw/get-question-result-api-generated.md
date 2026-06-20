## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 回應

回傳: [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_result_response.rs)

## 範例

[inline-code-attrs-start title = 'get_question_result 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_call() -> Result<(), Error> {
    let params: GetQuestionResultParams = GetQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-2026-07-poll-question-1".to_string(),
        include_details: Some(true),
        locale: Some("en-US".to_string()),
    };
    let result: GetQuestionResultResponse = get_question_result(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---