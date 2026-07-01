---
## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## 응답

반환: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantPackageResponse.h)

## 예시

[inline-code-attrs-start title = 'getTenantPackage 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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