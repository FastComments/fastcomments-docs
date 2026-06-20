## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlIdWS | string | Evet |  |
| userIds | string | Evet |  |

## Yanıt

Döndürür: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserPresenceStatusesResponse.h)

## Örnek

[inline-code-attrs-start title = 'getUserPresenceStatuses Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlIdWS = U("chat-room-77");
boost::optional<utility::string_t> optUserIds(U("alice@example.com,bob@example.com"));
api->getUserPresenceStatuses(tenantId, urlIdWS, optUserIds.value_or(U("")))
    .then([](pplx::task<std::shared_ptr<GetUserPresenceStatusesResponse>> t) {
        try {
            auto resp = t.get();
            if (!resp) resp = std::make_shared<GetUserPresenceStatusesResponse>();
            (void)resp;
        } catch (const std::exception&) {
            auto errResp = std::make_shared<GetUserPresenceStatusesResponse>();
            (void)errResp;
        }
    });
[inline-code-end]

---