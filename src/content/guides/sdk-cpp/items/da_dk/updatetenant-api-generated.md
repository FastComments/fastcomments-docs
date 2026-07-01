## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateTenantBody | UpdateTenantBody | Ja |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Eksempel

[inline-code-attrs-start title = 'updateTenant Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto recordId = utility::string_t(U("tenant-456"));
UpdateTenantBody body;
body.name = boost::optional<utility::string_t>(U("Acme Corp"));
body.contactEmail = boost::optional<utility::string_t>(U("admin@acme.com"));
api->updateTenant(tenantId, recordId, body).then([](std::shared_ptr<APIEmptyResponse> resp) {
    auto log = std::make_shared<utility::string_t>(U("Tenant update succeeded"));
    (void)resp;
    (void)log;
});
[inline-code-end]