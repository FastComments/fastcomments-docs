## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| question_id | String | 아니오 |  |
| question_ids | Vec<String> | 아니오 |  |
| url_id | String | 아니오 |  |
| start_date | String | 아니오 |  |
| force_recalculate | bool | 아니오 |  |
| min_value | f64 | 아니오 |  |
| max_value | f64 | 아니오 |  |
| limit | f64 | 아니오 |  |

## 응답

반환: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_comments_with_question_results_200_response.rs)

---