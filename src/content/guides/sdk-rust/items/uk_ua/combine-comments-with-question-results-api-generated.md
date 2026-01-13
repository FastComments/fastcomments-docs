## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| question_id | String | Ні |  |
| question_ids | Vec<String> | Ні |  |
| url_id | String | Ні |  |
| start_date | String | Ні |  |
| force_recalculate | bool | Ні |  |
| min_value | f64 | Ні |  |
| max_value | f64 | Ні |  |
| limit | f64 | Ні |  |

## Відповідь

Повертає: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_comments_with_question_results_200_response.rs)

---