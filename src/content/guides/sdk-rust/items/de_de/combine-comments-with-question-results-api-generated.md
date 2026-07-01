## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Yes |  |
| question_id | String | No |  |
| question_ids | Vec<String> | No |  |
| url_id | String | No |  |
| start_date | chrono::DateTime<chrono::FixedOffset> | No |  |
| force_recalculate | bool | No |  |
| min_value | f64 | No |  |
| max_value | f64 | No |  |
| limit | f64 | No |  |

## Antwort

Rückgabe: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_question_results_with_comments_response.rs)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für combine_comments_with_question_results'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---