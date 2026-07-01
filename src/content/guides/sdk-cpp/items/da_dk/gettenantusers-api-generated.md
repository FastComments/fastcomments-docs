## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | double | Nej |  |

## Svar

Returnerer: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantUsersResponse.h)

## Eksempel

[inline-code-attrs-start title = 'getTenantUsers Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<double> skip = 10;

api->getTenantUsers(tenantId, skip).then([](pplx::task<std::shared_ptr<GetTenantUsersResponse>> t){
    try {
        auto resp = t.get();
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---