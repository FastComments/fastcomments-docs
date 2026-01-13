## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| question_id | String | Όχι |  |
| question_ids | Vec<String> | Όχι |  |
| url_id | String | Όχι |  |
| time_bucket | models::AggregateTimeBucket | Όχι |  |
| start_date | String | Όχι |  |
| force_recalculate | bool | Όχι |  |

## Απόκριση

Επιστρέφει: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

---