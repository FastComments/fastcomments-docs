## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Tak |  |

## Odpowiedź

Zwraca: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład updateTenantPackage'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto body = std::make_shared<UpdateTenantPackageBody>();
body->packageId = utility::conversions::to_string_t("premium-plan");
body->expirationDate = utility::conversions::to_string_t("2025-12-31");
body->notes = boost::optional<utility::string_t>(utility::conversions::to_string_t("Upgraded package"));

api->updateTenantPackage(utility::conversions::to_string_t("my-tenant-123"),
                         utility::conversions::to_string_t("pkg-456"),
                         body)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task) {
        try {
            auto response = task.get();
            // obsługa sukcesu
        } catch (const std::exception& ex) {
            // obsługa błędu
        }
    });
[inline-code-end]