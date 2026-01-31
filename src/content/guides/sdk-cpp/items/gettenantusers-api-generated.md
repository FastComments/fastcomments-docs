## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | double | No |  |

## Response

Returns: [`GetTenantUsers_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantUsers_200_response.h)

## Example

[inline-code-attrs-start title = 'getTenantUsers Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 10.0;
api->getTenantUsers(tenantId, skip)
.then([](pplx::task<std::shared_ptr<GetTenantUsers_200_response>> task) {
    try {
        auto resp = task.get();
        auto result = resp ? std::make_shared<GetTenantUsers_200_response>(*resp)
                           : std::make_shared<GetTenantUsers_200_response>();
        (void)result;
    } catch (...) {
        auto empty = std::make_shared<GetTenantUsers_200_response>();
        (void)empty;
    }
});
[inline-code-end]
