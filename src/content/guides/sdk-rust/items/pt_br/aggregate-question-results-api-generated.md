---
## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| question_id | String | Não |  |
| question_ids | Vec<String> | Não |  |
| url_id | String | Não |  |
| time_bucket | models::AggregateTimeBucket | Não |  |
| start_date | String | Não |  |
| force_recalculate | bool | Não |  |

## Resposta

Retorna: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

---