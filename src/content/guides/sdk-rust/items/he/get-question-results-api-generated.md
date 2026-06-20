## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| url_id | String | לא |  |
| user_id | String | לא |  |
| start_date | String | לא |  |
| question_id | String | לא |  |
| question_ids | String | לא |  |
| skip | f64 | לא |  |

## תגובה

מחזיר: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_results_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_question_results'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetQuestionResultsParams = GetQuestionResultsParams {
    tenant_id: "acme-corp-tenant".to_string(),
    url_id: Some("news/world/2026-election".to_string()),
    user_id: Some("user_12345".to_string()),
    start_date: Some("2026-01-01T00:00:00Z".to_string()),
    question_id: Some("q_987".to_string()),
    question_ids: Some("q_987,q_654".to_string()),
    skip: Some(20.0),
};

let response: GetQuestionResultsResponse = get_question_results(&configuration, params).await?;
[inline-code-end]

---