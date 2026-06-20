## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| skip | f64 | לא |  |

## תגובה

מחזיר: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_configs_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_question_configs'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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