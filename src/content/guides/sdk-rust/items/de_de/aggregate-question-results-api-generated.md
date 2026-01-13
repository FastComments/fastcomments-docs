## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| question_id | String | Nein |  |
| question_ids | Vec<String> | Nein |  |
| url_id | String | Nein |  |
| time_bucket | models::AggregateTimeBucket | Nein |  |
| start_date | String | Nein |  |
| force_recalculate | bool | Nein |  |

## Antwort

Rückgabe: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

---