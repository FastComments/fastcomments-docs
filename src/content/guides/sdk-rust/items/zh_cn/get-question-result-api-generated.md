## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 响应

返回: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_result_200_response.rs)

## 示例

[inline-code-attrs-start title = 'get_question_result 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_question_result() -> Result<GetQuestionResult200Response, Error> {
    let params: GetQuestionResultParams = GetQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "question-12345".to_string(),
    };
    let _include_metadata: Option<bool> = Some(true);
    let response: GetQuestionResult200Response = get_question_result(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---