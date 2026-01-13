## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| question_id | String | Όχι |  |
| question_ids | Vec<String> | Όχι |  |
| url_id | String | Όχι |  |
| start_date | String | Όχι |  |
| force_recalculate | bool | Όχι |  |
| min_value | f64 | Όχι |  |
| max_value | f64 | Όχι |  |
| limit | f64 | Όχι |  |

## Απάντηση

Επιστρέφει: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_comments_with_question_results_200_response.rs)

---