## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantBody | CreateTenantBody | 是 |  |

## 回應

返回: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenantResponse.h)

## 範例

[inline-code-attrs-start title = 'createTenant 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
CreateTenantBody body;
body.setName(utility::conversions::to_string_t("Acme Corp"));
body.setAdminEmail(utility::conversions::to_string_t("admin@acme.com"));
body.setPlan(utility::conversions::to_string_t("enterprise"));
body.setDescription(boost::optional<utility::string_t>(utility::conversions::to_string_t("Primary tenant for Acme")));

api->createTenant(tenantId, body).then([](pplx::task<std::shared_ptr<CreateTenantResponse>> t){
    auto resp = t.get();
});
[inline-code-end]