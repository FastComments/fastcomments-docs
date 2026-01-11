## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| question_id | String | Nein |  |
| question_ids | Vec<String> | Nein |  |
| url_id | String | Nein |  |
| start_date | String | Nein |  |
| force_recalculate | bool | Nein |  |
| min_value | f64 | Nein |  |
| max_value | f64 | Nein |  |
| limit | f64 | Nein |  |

## Antwort

Gibt zurück: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_comments_with_question_results_200_response.rs)

---