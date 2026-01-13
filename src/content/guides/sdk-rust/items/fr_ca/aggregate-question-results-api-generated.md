## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| question_id | String | Non |  |
| question_ids | Vec<String> | Non |  |
| url_id | String | Non |  |
| time_bucket | models::AggregateTimeBucket | Non |  |
| start_date | String | Non |  |
| force_recalculate | bool | Non |  |

## Réponse

Renvoie: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

---