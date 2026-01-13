## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| question_id | String | Не |  |
| question_ids | Vec<String> | Не |  |
| url_id | String | Не |  |
| time_bucket | models::AggregateTimeBucket | Не |  |
| start_date | String | Не |  |
| force_recalculate | bool | Не |  |

## Отговор

Връща: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

---