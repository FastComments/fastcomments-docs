## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예제

[inline-code-attrs-start title = 'deletePendingWebhookEvent 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t eventId = U("wh_ev_987654321");
boost::optional<utility::string_t> requestId = U("req-20260619-01");
api->deletePendingWebhookEvent(tenantId, eventId).then([requestId](std::shared_ptr<APIEmptyResponse> resp) {
    if (resp) {
        auto ack = std::make_shared<APIEmptyResponse>(*resp);
    }
});
[inline-code-end]

---