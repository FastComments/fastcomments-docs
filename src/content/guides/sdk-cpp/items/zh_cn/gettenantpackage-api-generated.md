## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## 响应

返回: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantPackageResponse.h)

## 示例

[inline-code-attrs-start title = 'getTenantPackage 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto packageId = U("pkg-456");
api->getTenantPackage(tenantId, packageId).then([](pplx::task<std::shared_ptr<GetTenantPackageResponse>> task){
    try{
        auto resp = task.get();
        auto result = std::make_shared<GetTenantPackageResponse>(*resp);
    }catch(...){}
});
[inline-code-end]

---