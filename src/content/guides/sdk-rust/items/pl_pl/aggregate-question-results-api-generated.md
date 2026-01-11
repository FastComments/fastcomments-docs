## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| question_id | String | Nie |  |
| question_ids | Vec<String> | Nie |  |
| url_id | String | Nie |  |
| time_bucket | models::AggregateTimeBucket | Nie |  |
| start_date | String | Nie |  |
| force_recalculate | bool | Nie |  |

## Odpowiedź

Zwraca: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

---