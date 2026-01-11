## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| question_id | String | Nee |  |
| question_ids | Vec<String> | Nee |  |
| url_id | String | Nee |  |
| time_bucket | models::AggregateTimeBucket | Nee |  |
| start_date | String | Nee |  |
| force_recalculate | bool | Nee |  |

## Respons

Retourneert: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

---