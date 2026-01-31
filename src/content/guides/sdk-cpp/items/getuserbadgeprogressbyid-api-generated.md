## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetUserBadgeProgressById_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserBadgeProgressById_200_response.h)

## Example

[inline-code-attrs-start title = 'getUserBadgeProgressById Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> overrideTenant = boost::optional<utility::string_t>(U("my-tenant-override"));
utility::string_t tenantId = overrideTenant.value_or(U("my-tenant-123"));
utility::string_t id = U("user-456");
auto defaultResp = std::make_shared<GetUserBadgeProgressById_200_response>();
api->getUserBadgeProgressById(tenantId, id)
.then([](pplx::task<std::shared_ptr<GetUserBadgeProgressById_200_response>> task){
    try {
        auto resp = task.get();
        if(!resp) resp = std::make_shared<GetUserBadgeProgressById_200_response>();
    } catch(const std::exception &e) {
    }
});
[inline-code-end]
