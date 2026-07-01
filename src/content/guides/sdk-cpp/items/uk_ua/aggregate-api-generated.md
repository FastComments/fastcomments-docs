Агрегує документи шляхом групування їх (якщо вказано `groupBy`) та застосуванням декількох операцій. Підтримуються різні операції (наприклад, `sum`, `countDistinct`, `avg` тощо).

## Parameters

| Назва | Тип | Обов'язково | Опис |
|------|------|------------|------|
| tenantId | string | Так |  |
| aggregationRequest | AggregationRequest | Так |  |
| options | const AggregateOptions& | Так |  |

## Response

Повертає: [`AggregateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregateResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад агрегування'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
            // обробити відповідь
        } catch (const std::exception& e) {
            // обробити помилку
        }
    });
[inline-code-end]

---