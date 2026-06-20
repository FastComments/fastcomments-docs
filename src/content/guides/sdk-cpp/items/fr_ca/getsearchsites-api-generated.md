## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| value | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationSiteSearchResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de getSearchSites'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> valueOpt = boost::optional<utility::string_t>(U("my-tenant-123"));
boost::optional<utility::string_t> ssoOpt = boost::optional<utility::string_t>(U("user@example.com"));
api->getSearchSites(valueOpt, ssoOpt)
    .then([](std::shared_ptr<ModerationSiteSearchResponse> resp){
        auto response = resp ? resp : std::make_shared<ModerationSiteSearchResponse>();
        (void)response;
    })
    .wait();
[inline-code-end]

---