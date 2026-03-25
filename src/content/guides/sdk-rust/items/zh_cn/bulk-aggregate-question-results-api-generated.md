## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| bulk_aggregate_question_results_request | models::BulkAggregateQuestionResultsRequest | 是 |  |
| force_recalculate | bool | 否 |  |

## 响应

返回: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/bulk_aggregate_question_results_200_response.rs)

## 示例

[inline-code-attrs-start title = 'bulk_aggregate_question_results 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: BulkAggregateQuestionResultsParams = BulkAggregateQuestionResultsParams {
    tenant_id: "acme-corp-tenant".to_string(),
    bulk_aggregate_question_results_request: models::BulkAggregateQuestionResultsRequest {
        questions: vec![
            models::BulkAggregateQuestionItem {
                question_id: "news/article-2026-03-24-comments".to_string(),
                include_subquestions: Some(true),
                top_n: Some(5),
            }
        ],
        time_bucket: models::AggregateTimeBucket::Daily,
        start_time: "2026-03-01T00:00:00Z".to_string(),
        end_time: "2026-03-24T23:59:59Z".to_string(),
    },
    force_recalculate: Some(true),
};
let response: BulkAggregateQuestionResults200Response = bulk_aggregate_question_results(&configuration, params).await?;
[inline-code-end]

---