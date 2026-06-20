## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantPackageBody | CreateTenantPackageBody | 是 |  |

## 响应

返回：[`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenantPackageResponse.h)

## 示例

[inline-code-attrs-start title = 'createTenantPackage 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateTenantPackageBody body;
body.name = U("Premium Support");
body.contactEmail = U("admin@example.com");
body.seats = boost::optional<int>(25);
body.expiresAt = boost::optional<utility::string_t>(U("2026-12-31"));

api->createTenantPackage(tenantId, body)
.then([](std::shared_ptr<CreateTenantPackageResponse> resp){
    auto pkg = std::make_shared<CreateTenantPackageResponse>();
    if (resp) pkg = resp;
    return pkg;
})
.then([](std::shared_ptr<CreateTenantPackageResponse> finalResp){
    (void)finalResp;
});
[inline-code-end]

---