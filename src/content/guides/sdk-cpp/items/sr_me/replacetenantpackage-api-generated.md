## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Da |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Primjer

[inline-code-attrs-start title = 'Primjer replaceTenantPackage'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t packageId = U("pkg-456");
auto replaceBody = std::make_shared<ReplaceTenantPackageBody>();
replaceBody->name = U("Pro Plan");
replaceBody->description = boost::optional<utility::string_t>(U("Enterprise package with enhanced support"));
replaceBody->maxSeats = boost::optional<int>(500);
api->replaceTenantPackage(tenantId, packageId, *replaceBody)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task){
    try {
        auto resp = task.get();
        std::cout << "Package replaced successfully\n";
    } catch (const std::exception &e) {
        std::cerr << "Error replacing package: " << e.what() << '\n';
    }
});
[inline-code-end]

---