## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| usernameStartsWith | string | Nee |  |
| mentionGroupIds | vector<string | Nee |  |
| sso | string | Nee |  |
| searchSection | string | Nee |  |

## Reactie

Retourneert: [`SearchUsers_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SearchUsers_200_response.h)

## Voorbeeld

[inline-code-attrs-start title = 'searchUsers Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("https://example.com/articles/987");
boost::optional<utility::string_t> usernameStartsWith = boost::optional<utility::string_t>(U("alice"));
boost::optional<std::vector<utility::string_t>> mentionGroupIds = boost::optional<std::vector<utility::string_t>>({ U("editors"), U("authors") });
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<utility::string_t> searchSection = boost::optional<utility::string_t>(U("main"));

api->searchUsers(tenantId, urlId, usernameStartsWith, mentionGroupIds, sso, searchSection)
.then([](pplx::task<std::shared_ptr<SearchUsers_200_response>> t) {
    try {
        auto resp = t.get();
        auto copied = std::make_shared<SearchUsers_200_response>(*resp);
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---