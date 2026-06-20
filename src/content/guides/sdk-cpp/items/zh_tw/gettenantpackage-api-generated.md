## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

Returns: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantPackageResponse.h)

## 範例

[inline-code-attrs-start title = 'getTenantPackage 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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