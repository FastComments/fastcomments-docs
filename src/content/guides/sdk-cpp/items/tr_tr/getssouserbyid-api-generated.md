## Parametreler

| Ad | Tür | Gereki ri | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |

## Yanıt

Döndürür: [`GetSSOUserByIdAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSSOUserByIdAPIResponse.h)

## Örnek

[inline-code-attrs-start title = 'getSSOUserById Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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