## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|-------------|
| value | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationSiteSearchResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getSearchSites Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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