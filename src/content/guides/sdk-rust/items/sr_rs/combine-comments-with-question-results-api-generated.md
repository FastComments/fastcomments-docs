## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenant_id | String | Да |  |
| question_id | String | Не |  |
| question_ids | Vec<String> | Не |  |
| url_id | String | Не |  |
| start_date | chrono::DateTime<chrono::FixedOffset> | Не |  |
| force_recalculate | bool | Не |  |
| min_value | f64 | Не |  |
| max_value | f64 | Не |  |
| limit | f64 | Не |  |

## Одговор

Враћа: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_question_results_with_comments_response.rs)

## Пример

[inline-code-attrs-start title = 'combine_comments_with_question_results Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let params = CombineCommentsWithQuestionResultsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        question_id: Some("q123".to_string()),
        question_ids: Some(vec!["q123".to_string(), "q124".to_string()]),
        url_id: Some("news/article".to_string()),
        start_date: Some(chrono::DateTime::parse_from_rfc3339("2023-01-01T00:00:00+00:00").unwrap()),
        force_recalculate: Some(true),
        min_value: Some(0.0),
        max_value: Some(100.0),
        limit: Some(50.0),
    };
    let _response = combine_comments_with_question_results(&config, params).await?;
    Ok(())
}
[inline-code-end]