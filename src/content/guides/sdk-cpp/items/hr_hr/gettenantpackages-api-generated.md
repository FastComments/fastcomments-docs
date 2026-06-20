## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| skip | double | Ne |  |

## Odgovor

Vraća: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantPackagesResponse.h)

## Primjer

[inline-code-attrs-start title = 'Primjer getTenantPackages'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 20.0;
auto placeholder = std::make_shared<GetTenantPackagesResponse>();
api->getTenantPackages(tenantId, skip).then([placeholder](pplx::task<std::shared_ptr<GetTenantPackagesResponse>> t) {
    try {
        auto resp = t.get();
        std::cout << "Received packages: " << (resp ? "yes" : "no") << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Error fetching packages: " << e.what() << std::endl;
    }
});
[inline-code-end]