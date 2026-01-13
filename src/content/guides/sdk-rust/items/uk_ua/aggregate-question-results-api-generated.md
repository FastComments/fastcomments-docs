---
## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| question_id | String | Ні |  |
| question_ids | Vec<String> | Ні |  |
| url_id | String | Ні |  |
| time_bucket | models::AggregateTimeBucket | Ні |  |
| start_date | String | Ні |  |
| force_recalculate | bool | Ні |  |

## Відповідь

Повертає: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

---