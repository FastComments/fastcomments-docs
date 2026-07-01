## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vraća: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantUserResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer getTenantUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto userId = utility::conversions::to_string_t("user@example.com");
api->getTenantUser(tenantId, userId)
    .then([](pplx::task<std::shared_ptr<GetTenantUserResponse>> task) {
        try {
            auto response = task.get();
            // Koristite odgovor po potrebi
        } catch (const std::exception&) {
            // Obrada greške
        }
    });
[inline-code-end]