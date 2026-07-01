## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| updateTenantUserBody | UpdateTenantUserBody | Tak |  |
| updateComments | string | Nie |  |

## Odpowiedź

Zwraca: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Przykład

[inline-code-attrs-start title = 'updateTenantUser Przykład'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user-456");
UpdateTenantUserBody body;
body.email = U("john.doe@example.com");
body.role = U("admin");
boost::optional<utility::string_t> updateComments = U("Promoted to admin");

api->updateTenantUser(tenantId, userId, body, updateComments)
    .then([](std::shared_ptr<APIEmptyResponse>){ });
[inline-code-end]

---