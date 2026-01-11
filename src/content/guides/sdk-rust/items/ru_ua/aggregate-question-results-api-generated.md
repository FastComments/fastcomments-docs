## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| question_id | String | Нет |  |
| question_ids | Vec<String> | Нет |  |
| url_id | String | Нет |  |
| time_bucket | models::AggregateTimeBucket | Нет |  |
| start_date | String | Нет |  |
| force_recalculate | bool | Нет |  |

## Ответ

Возвращает: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

---