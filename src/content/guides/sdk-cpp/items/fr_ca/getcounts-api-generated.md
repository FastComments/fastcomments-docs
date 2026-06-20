## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| sso | string | Non |  |

## Réponse

Renvoie : [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetBannedUsersCountResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCounts'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("my-tenant-123"));
api->getCounts(sso).then([](pplx::task<std::shared_ptr<GetBannedUsersCountResponse>> task) {
    try {
        auto resp = task.get();
        if(!resp) resp = std::make_shared<GetBannedUsersCountResponse>();
    } catch(...) {
    }
});
[inline-code-end]

---