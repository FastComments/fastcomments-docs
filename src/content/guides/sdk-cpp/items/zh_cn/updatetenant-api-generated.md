## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateTenantBody | UpdateTenantBody | 是 |  |

## 响应

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 示例

[inline-code-attrs-start title = 'updateTenant 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("admin-user-456");
auto updateTenantBody = std::make_shared<UpdateTenantBody>();
updateTenantBody->name = U("Acme Corporation");
updateTenantBody->ownerEmail = boost::optional<utility::string_t>(U("owner@acme.com"));
updateTenantBody->isActive = boost::optional<bool>(true);
api->updateTenant(tenantId, id, updateTenantBody)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t) {
    try {
        auto resp = t.get();
        (void)resp;
    } catch (const std::exception&) {
    }
});
[inline-code-end]