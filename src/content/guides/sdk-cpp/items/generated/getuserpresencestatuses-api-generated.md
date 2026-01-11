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
utility::string_t urlIdWS = U("wss://comments.fastcomments.com/ws/room-456");
boost::optional<utility::string_t> optUserIds = U("alice@example.com,bob@example.com");
utility::string_t userIds = optUserIds ? *optUserIds : utility::string_t();

api->getUserPresenceStatuses(tenantId, urlIdWS, userIds)
    .then([](pplx::task<std::shared_ptr<GetUserPresenceStatuses_200_response>> task){
        try {
            auto resp = task.get();
            if(!resp) resp = std::make_shared<GetUserPresenceStatuses_200_response>();
            // use resp as needed
        } catch (const std::exception&) {
            auto resp = std::make_shared<GetUserPresenceStatuses_200_response>();
        }
    });
[inline-code-end]
