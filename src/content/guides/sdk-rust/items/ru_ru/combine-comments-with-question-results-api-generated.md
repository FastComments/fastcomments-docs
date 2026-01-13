## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| question_id | String | Нет |  |
| question_ids | Vec<String> | Нет |  |
| url_id | String | Нет |  |
| start_date | String | Нет |  |
| force_recalculate | bool | Нет |  |
| min_value | f64 | Нет |  |
| max_value | f64 | Нет |  |
| limit | f64 | Нет |  |

## Ответ

Возвращает: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_comments_with_question_results_200_response.rs)

---