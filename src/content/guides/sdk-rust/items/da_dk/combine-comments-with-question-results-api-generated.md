## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| question_id | String | Nej |  |
| question_ids | Vec<String> | Nej |  |
| url_id | String | Nej |  |
| start_date | chrono::DateTime<chrono::FixedOffset> | Nej |  |
| force_recalculate | bool | Nej |  |
| min_value | f64 | Nej |  |
| max_value | f64 | Nej |  |
| limit | f64 | Nej |  |

## Svar

Returnerer: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_question_results_with_comments_response.rs)

## Eksempel

[inline-code-attrs-start title = 'combine_comments_with_question_results Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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