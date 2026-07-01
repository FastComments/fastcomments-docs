## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Yes |  |
| forceRecalculate | bool | No |  |

## 响应

返回：[`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BulkAggregateQuestionResultsResponse.h)

## 示例

[inline-code-attrs-start title = 'bulkAggregateQuestionResults 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
BulkAggregateQuestionResultsRequest request;
request.questionIds = {
    utility::conversions::to_string_t("q123"),
    utility::conversions::to_string_t("q456")
};
request.startDate = utility::datetime::from_string(U("2023-01-01T00:00:00Z"));
request.endDate = utility::datetime::from_string(U("2023-01-31T23:59:59Z"));
boost::optional<bool> forceRecalc = true;
api->bulkAggregateQuestionResults(tenantId, request, forceRecalc)
   .then([](pplx::task<std::shared_ptr<BulkAggregateQuestionResultsResponse>> t) {
       auto resp = t.get();
   });
[inline-code-end]