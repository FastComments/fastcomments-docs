---
聚合文档，通过（如果提供了 groupBy）进行分组并应用多种操作。支持不同的操作（例如 sum、countDistinct、avg 等）。

## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | Yes |  |
| options | const AggregateOptions& | Yes |  |

## 响应

返回：[`AggregateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregateResponse.h)

## 示例

[inline-code-attrs-start title = '聚合 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");

AggregationRequest aggregationRequest;
aggregationRequest.setMetric(U("commentCount"));
aggregationRequest.setStartDate(U("2023-01-01T00:00:00Z"));
aggregationRequest.setEndDate(U("2023-12-31T23:59:59Z"));
aggregationRequest.setFilters({ U("status:approved") });

AggregateOptions options;
options.limit = boost::optional<int>(100);
options.includeMetadata = boost::optional<bool>(true);

api->aggregate(tenantId, aggregationRequest, options)
    .then([](pplx::task<std::shared_ptr<AggregateResponse>> task) {
        try {
            auto response = task.get();
            // process response
        } catch (const std::exception& e) {
            // handle error
        }
    });
[inline-code-end]