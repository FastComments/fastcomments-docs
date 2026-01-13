## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlIdWS | string | 예 |  |
| userIds | string | 예 |  |

## 응답

반환: [`GetUserPresenceStatuses_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserPresenceStatuses_200_response.h)

## 예제

[inline-code-attrs-start title = 'getUserPresenceStatuses 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlIdWS = U("wss://realtime.fastcomments.com/socket/tenant-abc");
boost::optional<utility::string_t> optUserIds = U("alice@example.com,bob@example.com");
utility::string_t userIds = optUserIds.value_or(U("alice@example.com"));
auto task = api->getUserPresenceStatuses(tenantId, urlIdWS, userIds)
    .then([](std::shared_ptr<GetUserPresenceStatuses_200_response> resp) {
        if (!resp) return;
        auto copy = std::make_shared<GetUserPresenceStatuses_200_response>(*resp);
    });
task.wait();
[inline-code-end]

---