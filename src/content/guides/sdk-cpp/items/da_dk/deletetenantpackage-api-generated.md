## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Svar

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Eksempel

[inline-code-attrs-start title = 'deleteTenantPackage Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto packageId = utility::conversions::to_string_t("pkg-456");
boost::optional<utility::string_t> optTenant = tenantId;
boost::optional<utility::string_t> optPackage = packageId;
api->deleteTenantPackage(optTenant.value(), optPackage.value())
    .then([](std::shared_ptr<APIEmptyResponse>){ })
    .then([](pplx::task<void> t){ try{ t.get(); }catch(const std::exception&){ } });
[inline-code-end]