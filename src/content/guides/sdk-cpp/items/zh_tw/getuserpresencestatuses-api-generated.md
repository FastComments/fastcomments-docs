## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlIdWS | string | 是 |  |
| userIds | string | 是 |  |

## 回應

回傳: [`GetUserPresenceStatuses_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserPresenceStatuses_200_response.h)

## 範例

[inline-code-attrs-start title = 'getUserPresenceStatuses 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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