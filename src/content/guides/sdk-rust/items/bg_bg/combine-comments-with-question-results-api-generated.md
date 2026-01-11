## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| question_id | String | Не |  |
| question_ids | Vec<String> | Не |  |
| url_id | String | Не |  |
| start_date | String | Не |  |
| force_recalculate | bool | Не |  |
| min_value | f64 | Не |  |
| max_value | f64 | Не |  |
| limit | f64 | Не |  |

## Отговор

Връща: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_comments_with_question_results_200_response.rs)

---