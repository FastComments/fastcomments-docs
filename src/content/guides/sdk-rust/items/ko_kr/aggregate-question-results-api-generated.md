## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| question_id | String | 아니요 |  |
| question_ids | Vec<String> | 아니요 |  |
| url_id | String | 아니요 |  |
| time_bucket | models::AggregateTimeBucket | 아니요 |  |
| start_date | String | 아니요 |  |
| force_recalculate | bool | 아니요 |  |

## 응답

반환: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

---