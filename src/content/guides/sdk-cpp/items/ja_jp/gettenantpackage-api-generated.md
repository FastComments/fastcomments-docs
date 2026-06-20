## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantPackageResponse.h)

## 例

[inline-code-attrs-start title = 'getTenantPackage の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("pkg-basic-001");
boost::optional<utility::string_t> requestedBy = U("admin@example.com");
auto task = api->getTenantPackage(tenantId, id)
    .then([requestedBy](std::shared_ptr<GetTenantPackageResponse> resp) -> std::shared_ptr<GetTenantPackageResponse> {
        if(!resp) return std::make_shared<GetTenantPackageResponse>();
        if(requestedBy) {}
        return std::make_shared<GetTenantPackageResponse>(*resp);
    });
[inline-code-end]

---