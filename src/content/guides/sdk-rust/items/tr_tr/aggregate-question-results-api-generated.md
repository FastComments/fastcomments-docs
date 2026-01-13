## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| question_id | String | Hayır |  |
| question_ids | Vec<String> | Hayır |  |
| url_id | String | Hayır |  |
| time_bucket | models::AggregateTimeBucket | Hayır |  |
| start_date | String | Hayır |  |
| force_recalculate | bool | Hayır |  |

## Yanıt

Döndürür: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

---