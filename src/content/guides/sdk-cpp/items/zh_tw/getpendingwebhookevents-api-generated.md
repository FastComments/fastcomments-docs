## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 否 |  |
| externalId | string | 否 |  |
| eventType | string | 否 |  |
| type | string | 否 |  |
| domain | string | 否 |  |
| attemptCountGT | double | 否 |  |
| skip | double | 否 |  |

## 回應

回傳: [`GetPendingWebhookEvents_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPendingWebhookEvents_200_response.h)

## 範例

[inline-code-attrs-start title = 'getPendingWebhookEvents 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> commentId = boost::optional<utility::string_t>(U("cmt-98765"));
boost::optional<utility::string_t> externalId = boost::optional<utility::string_t>(U("ext-abc-01"));
boost::optional<utility::string_t> eventType = boost::optional<utility::string_t>(U("comment_created"));
boost::optional<utility::string_t> type = boost::optional<utility::string_t>(U("webhook"));
boost::optional<utility::string_t> domain = boost::optional<utility::string_t>(U("example.com"));
boost::optional<double> attemptCountGT = boost::optional<double>(2.0);
boost::optional<double> skip = boost::optional<double>(0.0);

api->getPendingWebhookEvents(tenantId, commentId, externalId, eventType, type, domain, attemptCountGT, skip)
.then([](pplx::task<std::shared_ptr<GetPendingWebhookEvents_200_response>> t) {
    try {
        auto resp = t.get();
        auto copy = std::make_shared<GetPendingWebhookEvents_200_response>(*resp);
        (void)copy;
    } catch (const std::exception& e) {
        (void)e;
        auto empty = std::make_shared<GetPendingWebhookEvents_200_response>();
        (void)empty;
    }
});
[inline-code-end]

---