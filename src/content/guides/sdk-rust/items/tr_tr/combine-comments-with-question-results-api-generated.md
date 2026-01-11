## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| question_id | String | Hayır |  |
| question_ids | Vec<String> | Hayır |  |
| url_id | String | Hayır |  |
| start_date | String | Hayır |  |
| force_recalculate | bool | Hayır |  |
| min_value | f64 | Hayır |  |
| max_value | f64 | Hayır |  |
| limit | f64 | Hayır |  |

## Yanıt

Döndürür: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_comments_with_question_results_200_response.rs)

---