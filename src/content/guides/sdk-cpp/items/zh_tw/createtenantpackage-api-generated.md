## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantPackageBody | CreateTenantPackageBody | Yes |  |

## 回應

返回：[`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenantPackageResponse.h)

## 範例

[inline-code-attrs-start title = 'createTenantPackage 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
CreateTenantPackageBody body;
body.name = utility::conversions::to_string_t("Pro Plan");
body.maxUsers = 100;
body.expiryDate = boost::optional<utility::datetime>(utility::datetime::utc_now() + std::chrono::hours(24 * 30));
body.notes = boost::none;
api->createTenantPackage(tenantId, body).then([](pplx::task<std::shared_ptr<CreateTenantPackageResponse>> t){
    try{
        auto resp = t.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]