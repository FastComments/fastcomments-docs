## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## レスポンス

返り値: [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_result_response.rs)

## 例

[inline-code-attrs-start title = 'get_question_result の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_question_result(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "question-12345".to_string(),
        locale: Some("en-US".to_string()),
    };
    let _response: GetQuestionResultResponse = get_question_result(config, params).await?;
    Ok(())
}
[inline-code-end]