## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlIdWS | string | Yes |  |
| userIds | string | Yes |  |

## Response

Returns: [`GetUserPresenceStatuses_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserPresenceStatuses_200_response.h)

## Example

[inline-code-attrs-start title = 'getUserPresenceStatuses Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlIdWS = U("comments-websocket");
boost::optional<utility::string_t> userIdsOpt = boost::optional<utility::string_t>(U("alice@example.com,bob@example.com"));
api->getUserPresenceStatuses(tenantId, urlIdWS, *userIdsOpt).then([](pplx::task<std::shared_ptr<GetUserPresenceStatuses_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto snapshot = std::make_shared<GetUserPresenceStatuses_200_response>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]
