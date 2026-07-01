## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio getTenant'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto id = utility::conversions::to_string_t("tenant-admin-456");
boost::optional<utility::string_t> includeDetails = utility::conversions::to_string_t("full");
api->getTenant(tenantId, id)
    .then([](std::shared_ptr<GetTenantResponse> resp) {
        if (resp) {
            std::wcout << resp->toString() << std::endl;
        }
    });
[inline-code-end]