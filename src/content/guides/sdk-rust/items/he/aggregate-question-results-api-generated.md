## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| question_id | String | לא |  |
| question_ids | Vec<String> | לא |  |
| url_id | String | לא |  |
| time_bucket | models::AggregateTimeBucket | לא |  |
| start_date | String | לא |  |
| force_recalculate | bool | לא |  |

## תשובה

מחזיר: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

---