## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 响应

返回: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_result_200_response.rs)

## 示例

[inline-code-attrs-start title = 'get_question_result 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetQuestionResult200Response, Error> {
    let include_metadata: Option<bool> = Some(true);
    let params: GetQuestionResultParams = GetQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article/2026/12345".to_string(),
    };
    let response: GetQuestionResult200Response = get_question_result(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---