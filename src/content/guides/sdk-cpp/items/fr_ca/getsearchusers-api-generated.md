## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| value | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationUserSearchResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de getSearchUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> value(boost::optional<utility::string_t>(U("john.doe@example.com")));
boost::optional<utility::string_t> sso(boost::optional<utility::string_t>(U("my-tenant-123")));
api->getSearchUsers(value, sso).then([](std::shared_ptr<ModerationUserSearchResponse> resp){
    if (!resp) return;
    auto copy = std::make_shared<ModerationUserSearchResponse>(*resp);
});
[inline-code-end]

---