## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetTenant_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenant_200_response.h)

## Example

[inline-code-attrs-start title = 'getTenant Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> if_none_match = utility::string_t(U("W/\"12345\""));
auto tenantId = utility::string_t(U("my-tenant-123"));
auto userId = utility::string_t(U("user@example.com"));
api->getTenant(tenantId, userId)
.then([=](std::shared_ptr<GetTenant_200_response> resp) {
    if (resp) {
        auto tenant = std::make_shared<GetTenant_200_response>(*resp);
        (void)tenant;
    }
});
[inline-code-end]
