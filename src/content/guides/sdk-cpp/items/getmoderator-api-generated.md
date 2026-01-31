## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetModerator_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetModerator_200_response.h)

## Example

[inline-code-attrs-start title = 'getModerator Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenant = U("my-tenant-123");
utility::string_t moderatorId = U("moderator-456");
boost::optional<utility::string_t> tenantOpt(tenant);
api->getModerator(tenantOpt.value(), moderatorId)
.then([](pplx::task<std::shared_ptr<GetModerator_200_response>> task) {
    try {
        auto resp = task.get();
        auto result = resp ? resp : std::make_shared<GetModerator_200_response>();
    } catch (const std::exception& e) {
    }
});
[inline-code-end]
