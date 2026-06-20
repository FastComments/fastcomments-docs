## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| meta | string | Hayır |  |
| skip | double | Hayır |  |

## Yanıt

Döndürür: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantsResponse.h)

## Örnek

[inline-code-attrs-start title = 'getTenants Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> meta(U("user@example.com"));
boost::optional<double> skip(10.0);
api->getTenants(U("my-tenant-123"), meta, skip)
    .then([](std::shared_ptr<GetTenantsResponse> resp) {
        auto out = resp ? resp : std::make_shared<GetTenantsResponse>();
        if (resp) std::cout << "Fetched tenants successfully\n";
        else std::cout << "No tenants returned, using default\n";
    });
[inline-code-end]