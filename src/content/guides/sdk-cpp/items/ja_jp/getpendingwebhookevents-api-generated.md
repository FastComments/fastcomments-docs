## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| options | const GetPendingWebhookEventsOptions& | はい |  |

## レスポンス

戻り値: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPendingWebhookEventsResponse.h)

## 例

[inline-code-attrs-start title = 'getPendingWebhookEvents の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
GetPendingWebhookEventsOptions opts;
opts.pageSize = boost::optional<int>(100);
opts.startAfter = boost::optional<utility::string_t>(U("event-abc-456"));
api->getPendingWebhookEvents(tenantId, opts)
    .then([](pplx::task<std::shared_ptr<GetPendingWebhookEventsResponse>> task) {
        try {
            auto resp = task.get();
            auto copy = std::make_shared<GetPendingWebhookEventsResponse>(*resp);
        } catch (const std::exception&) {}
    });
[inline-code-end]