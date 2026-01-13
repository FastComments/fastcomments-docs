## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| question_id | String | Non |  |
| question_ids | Vec<String> | Non |  |
| url_id | String | Non |  |
| start_date | String | Non |  |
| force_recalculate | bool | Non |  |
| min_value | f64 | Non |  |
| max_value | f64 | Non |  |
| limit | f64 | Non |  |

## Réponse

Retourne: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_comments_with_question_results_200_response.rs)

---