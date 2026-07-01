## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Rückgabe

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Beispiel

[inline-code-attrs-start title = 'deleteTenantPackage Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto packageId = utility::conversions::to_string_t("pkg-456");
boost::optional<utility::string_t> optTenant = tenantId;
boost::optional<utility::string_t> optPackage = packageId;
api->deleteTenantPackage(optTenant.value(), optPackage.value())
    .then([](std::shared_ptr<APIEmptyResponse>){ })
    .then([](pplx::task<void> t){ try{ t.get(); }catch(const std::exception&){ } });
[inline-code-end]