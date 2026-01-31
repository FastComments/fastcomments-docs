## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | double | No |  |

## Response

Returns: [`GetTenantPackages_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantPackages_200_response.h)

## Example

[inline-code-attrs-start title = 'getTenantPackages Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 10.0;
auto fallback = std::make_shared<GetTenantPackages_200_response>();
api->getTenantPackages(tenantId, skip)
    .then([fallback](pplx::task<std::shared_ptr<GetTenantPackages_200_response>> task) {
        try {
            auto resp = task.get();
            auto result = resp ? resp : fallback;
            (void)result;
        } catch (const std::exception& e) {
            (void)e;
        }
    });
[inline-code-end]
