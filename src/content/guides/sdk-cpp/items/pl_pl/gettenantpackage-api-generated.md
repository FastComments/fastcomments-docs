## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Odpowiedź

Zwraca: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantPackageResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getTenantPackage'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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