---
## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| usernameStartsWith | string | Nej |  |
| mentionGroupIds | vector<string | Nej |  |
| sso | string | Nej |  |
| searchSection | string | Nej |  |

## Svar

Returnerer: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SearchUsersResult.h)

## Eksempel

[inline-code-attrs-start title = 'searchUsers Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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

---