## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| question_id | String | Nie |  |
| question_ids | Vec<String> | Nie |  |
| url_id | String | Nie |  |
| start_date | String | Nie |  |
| force_recalculate | bool | Nie |  |
| min_value | f64 | Nie |  |
| max_value | f64 | Nie |  |
| limit | f64 | Nie |  |

## Odpowiedź

Zwraca: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_comments_with_question_results_200_response.rs)

---