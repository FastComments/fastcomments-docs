## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantPackageBody | CreateTenantPackageBody | Yes |  |

## Response

Returns: [`CreateTenantPackage_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenantPackage_200_response.h)

## Example

[inline-code-attrs-start title = 'createTenantPackage Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto bodyPtr = std::make_shared<CreateTenantPackageBody>();
bodyPtr->packageName = utility::string_t(U("Premium Package"));
bodyPtr->seatCount = 25;
bodyPtr->contactEmail = boost::optional<utility::string_t>(U("billing@mycompany.com"));
bodyPtr->autoRenew = boost::optional<bool>(true);
api->createTenantPackage(tenantId, *bodyPtr).then([](pplx::task<std::shared_ptr<CreateTenantPackage_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) std::cout << "Package created: " << resp->packageId << std::endl;
    } catch (const std::exception &e) {
        std::cerr << "Create failed: " << e.what() << std::endl;
    }
});
[inline-code-end]
