---
## 参数

| 名称 | 类型 | 必需 | 说明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| update_question_result_body | models::UpdateQuestionResultBody | 是 |  |

## 响应

返回：[`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 示例

[inline-code-attrs-start title = 'update_question_result 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_update_question_result() -> Result<(), Error> {
    let body: models::UpdateQuestionResultBody = models::UpdateQuestionResultBody {
        answered: Some(true),
        confidence: Some(0.92),
        responder: Some("editor-zoe".to_string()),
        notes: Some("Validated against article sources".to_string()),
    };
    let params: UpdateQuestionResultParams = UpdateQuestionResultParams {
        tenant_id: "acme-news-tenant".to_string(),
        id: "news/article/5621/question/12".to_string(),
        update_question_result_body: body,
    };
    let _resp: ApiEmptyResponse = update_question_result(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---