## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| question_id | String | Ne |  |
| question_ids | Vec<String> | Ne |  |
| url_id | String | Ne |  |
| time_bucket | models::AggregateTimeBucket | Ne |  |
| start_date | String | Ne |  |
| force_recalculate | bool | Ne |  |

## Odgovor

Vrne: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

---