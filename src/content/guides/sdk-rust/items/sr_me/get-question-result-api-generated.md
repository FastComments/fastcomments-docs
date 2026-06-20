## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |

## Одговор

Враћа: [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_result_response.rs)

## Пример

[inline-code-attrs-start title = 'get_question_result Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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