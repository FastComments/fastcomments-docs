## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantPackageBody | CreateTenantPackageBody | 是 |  |

## 回應

回傳: [`CreateTenantPackage_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenantPackage_200_response.h)

## 範例

[inline-code-attrs-start title = 'createTenantPackage 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto body = std::make_shared<CreateTenantPackageBody>();
body->name = U("Standard Package - 1k comments");
body->createdByEmail = U("admin@mycompany.com");
body->monthlyLimit = boost::optional<int>(1000);
body->notes = boost::optional<utility::string_t>(U("Onboarding promo"));
api->createTenantPackage(tenantId, *body)
.then([](pplx::task<std::shared_ptr<CreateTenantPackage_200_response>> t) {
    try {
        auto resp = t.get();
        if (resp) {
            std::cout << "Package created: " << utility::conversions::to_utf8string(resp->packageId) << std::endl;
        }
    } catch (const std::exception& e) {
        std::cerr << "Create failed: " << e.what() << std::endl;
    }
});
[inline-code-end]

---