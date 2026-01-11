## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| question_id | String | 否 |  |
| question_ids | Vec<String> | 否 |  |
| url_id | String | 否 |  |
| start_date | String | 否 |  |
| force_recalculate | bool | 否 |  |
| min_value | f64 | 否 |  |
| max_value | f64 | 否 |  |
| limit | f64 | 否 |  |

## 响应

返回：[`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_comments_with_question_results_200_response.rs)

---