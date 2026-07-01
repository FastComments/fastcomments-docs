## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| id | string | Tak |  |

## Odpowiedź

Returns: [`GetSSOUserByIdAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSSOUserByIdAPIResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getSSOUserById'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto ssoUserId = U("user-789");
api->getSSOUserById(tenantId, ssoUserId)
    .then([](std::shared_ptr<GetSSOUserByIdAPIResponse> resp) {
        boost::optional<utility::string_t> email;
        if (resp && resp->email) email = resp->email;
        if (email) {
            auto e = *email;
        }
    });
[inline-code-end]