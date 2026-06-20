## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Yes |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 範例

[inline-code-attrs-start title = 'replaceTenantPackage 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t packageId = U("pkg-456");
auto replaceBody = std::make_shared<ReplaceTenantPackageBody>();
replaceBody->name = U("Pro Plan");
replaceBody->description = boost::optional<utility::string_t>(U("Enterprise package with enhanced support"));
replaceBody->maxSeats = boost::optional<int>(500);
api->replaceTenantPackage(tenantId, packageId, *replaceBody)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task){
    try {
        auto resp = task.get();
        std::cout << "Package replaced successfully\n";
    } catch (const std::exception &e) {
        std::cerr << "Error replacing package: " << e.what() << '\n';
    }
});
[inline-code-end]