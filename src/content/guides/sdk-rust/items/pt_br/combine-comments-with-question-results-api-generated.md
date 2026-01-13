## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| question_id | String | Não |  |
| question_ids | Vec<String> | Não |  |
| url_id | String | Não |  |
| start_date | String | Não |  |
| force_recalculate | bool | Não |  |
| min_value | f64 | Não |  |
| max_value | f64 | Não |  |
| limit | f64 | Não |  |

## Resposta

Retorna: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_comments_with_question_results_200_response.rs)

---