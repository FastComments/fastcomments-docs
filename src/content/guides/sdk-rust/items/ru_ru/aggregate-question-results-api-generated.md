## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| question_id | String | Нет |  |
| question_ids | Vec<String> | Нет |  |
| url_id | String | Нет |  |
| time_bucket | models::AggregateTimeBucket | Нет |  |
| start_date | String | Нет |  |
| force_recalculate | bool | Нет |  |

## Ответ

Возвращает: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример aggregate_question_results'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: AggregateQuestionResultsParams = AggregateQuestionResultsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        question_id: Some("satisfaction-8".to_string()),
        question_ids: Some(vec!["satisfaction-8".to_string(), "recommendation-3".to_string()]),
        url_id: Some("news/article/2026/ai-announce".to_string()),
        time_bucket: Some(models::AggregateTimeBucket::Daily),
        start_date: Some("2026-03-01T00:00:00Z".to_string()),
        force_recalculate: Some(true),
    };
    let aggregation: AggregateQuestionResults200Response = aggregate_question_results(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---