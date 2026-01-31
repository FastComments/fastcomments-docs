## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetTenantUser_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantUser_200_response.h)

## Example

[inline-code-attrs-start title = 'getTenantUser Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> includeProfile = utility::string_t(U("en-US"));
api->getTenantUser(tenantId, userId)
.then([](pplx::task<std::shared_ptr<GetTenantUser_200_response>> t) {
    try {
        auto resp = t.get();
        return std::make_shared<GetTenantUser_200_response>(*resp);
    } catch (...) {
        throw;
    }
})
.wait();
[inline-code-end]
