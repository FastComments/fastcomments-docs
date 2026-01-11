## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| question_id | String | Nej |  |
| question_ids | Vec<String> | Nej |  |
| url_id | String | Nej |  |
| time_bucket | models::AggregateTimeBucket | Nej |  |
| start_date | String | Nej |  |
| force_recalculate | bool | Nej |  |

## Svar

Returnerer: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

---