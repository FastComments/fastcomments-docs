## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| question_id | String | Όχι |  |
| question_ids | Vec<String> | Όχι |  |
| url_id | String | Όχι |  |
| start_date | chrono::DateTime<chrono::FixedOffset> | Όχι |  |
| force_recalculate | bool | Όχι |  |
| min_value | f64 | Όχι |  |
| max_value | f64 | Όχι |  |
| limit | f64 | Όχι |  |

## Απόκριση

Επιστρέφει: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_question_results_with_comments_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα combine_comments_with_question_results'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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