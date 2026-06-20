## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| skip | f64 | 否 |  |

## 回傳

回傳：[`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_configs_response.rs)

## 範例

[inline-code-attrs-start title = 'get_question_configs 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_question_configs() -> Result<GetQuestionConfigsResponse, Error> {
    let params: GetQuestionConfigsParams = GetQuestionConfigsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(20.0),
    };
    let response: GetQuestionConfigsResponse = get_question_configs(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---