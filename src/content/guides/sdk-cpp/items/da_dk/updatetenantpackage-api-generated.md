## Parametre

| Navn | Type | Krævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Yes |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Eksempel

[inline-code-attrs-start title = 'updateTenantPackage Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
            // håndtering af succes
        } catch (const std::exception& ex) {
            // håndtering af fejl
        }
    });
[inline-code-end]