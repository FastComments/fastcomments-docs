## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetTenantPackage_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantPackage_200_response.h)

## Example

[inline-code-attrs-start title = 'getTenantPackage Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t packageId = U("pkg-456");
boost::optional<utility::string_t> includeHistory = boost::none;
api->getTenantPackage(tenantId, packageId)
.then([](pplx::task<std::shared_ptr<GetTenantPackage_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto placeholder = std::make_shared<GetTenantPackage_200_response>();
            std::cout << "Package retrieved for tenant\n";
        } else {
            std::cout << "Package not found\n";
        }
    } catch (const std::exception &e) {
        std::cerr << "Request failed: " << e.what() << '\n';
    }
});
[inline-code-end]
