## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| usernameStartsWith | string | Non |  |
| mentionGroupIds | vector<string | Non |  |
| sso | string | Non |  |
| searchSection | string | Non |  |

## Réponse

Renvoie : [`SearchUsersResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SearchUsersResult.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de searchUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("/articles/2026/optimizing-cpp");
boost::optional<utility::string_t> usernameStartsWith(U("alice"));
std::vector<boost::optional<utility::string_t>> mentionGroupIds{
    boost::optional<utility::string_t>(U("editors")),
    boost::optional<utility::string_t>(U("reviewers"))
};
boost::optional<utility::string_t> sso(U("sso-jwt-42"));
boost::optional<utility::string_t> searchSection(U("comments"));

api->searchUsers(tenantId, urlId, usernameStartsWith, mentionGroupIds, sso, searchSection)
.then([](pplx::task<std::shared_ptr<SearchUsersResult>> task){
    try {
        auto res = task.get();
        auto finalRes = res ? res : std::make_shared<SearchUsersResult>();
        (void)finalRes;
    } catch (const std::exception&) {
    }
});
[inline-code-end]