## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| url_id | String | 아니요 |  |
| user_id | String | 아니요 |  |
| start_date | String | 아니요 |  |
| question_id | String | 아니요 |  |
| question_ids | String | 아니요 |  |
| skip | f64 | 아니요 |  |

## 응답

반환: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_results_200_response.rs)

## 예제

[inline-code-attrs-start title = 'get_question_results 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_question_results() -> Result<(), Error> {
    let params = GetQuestionResultsParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: Some(String::from("news/article/2026/01/12/breaking")),
        user_id: Some(String::from("user-98765")),
        start_date: Some(String::from("2025-12-01")),
        question_id: Some(String::from("q-42")),
        question_ids: Some(String::from("q-42,q-43")),
        skip: Some(10.0),
    };
    let results: GetQuestionResults200Response = get_question_results(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---