## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |

## Odgovor

Vrača: [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_result_response.rs)

## Primer

[inline-code-attrs-start title = 'get_question_result Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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