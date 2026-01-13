## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 아니요 |  |
| externalId | string | 아니요 |  |
| eventType | string | 아니요 |  |
| type | string | 아니요 |  |
| domain | string | 아니요 |  |
| attemptCountGT | double | 아니요 |  |
| skip | double | 아니요 |  |

## 응답

반환: [`GetPendingWebhookEvents_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPendingWebhookEvents_200_response.h)

## 예제

[inline-code-attrs-start title = 'getPendingWebhookEvents 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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