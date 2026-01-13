## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| question_id | String | Ne |  |
| question_ids | Vec<String> | Ne |  |
| url_id | String | Ne |  |
| start_date | String | Ne |  |
| force_recalculate | bool | Ne |  |
| min_value | f64 | Ne |  |
| max_value | f64 | Ne |  |
| limit | f64 | Ne |  |

## Odgovor

Vraća: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_comments_with_question_results_200_response.rs)

---