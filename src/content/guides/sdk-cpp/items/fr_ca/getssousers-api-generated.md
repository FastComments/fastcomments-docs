## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| skip | int32_t | Non |  |

## Réponse

Retourne: [`GetSSOUsers_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSSOUsers_200_response.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de getSSOUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<int32_t> skip = 10;
api->getSSOUsers(tenantId, skip).then([](pplx::task<std::shared_ptr<GetSSOUsers_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto copy = std::make_shared<GetSSOUsers_200_response>(*resp);
        }
    } catch(...) {}
});
[inline-code-end]

---