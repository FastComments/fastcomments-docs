## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| usernameStartsWith | string | לא |  |
| mentionGroupIds | vector<string | לא |  |
| sso | string | לא |  |
| searchSection | string | לא |  |

## תגובה

מחזיר: [`SearchUsers_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SearchUsers_200_response.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמת searchUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("/articles/2026/fast-comments-integration");
boost::optional<utility::string_t> usernameStartsWith = boost::optional<utility::string_t>(U("alex"));
std::vector<utility::string_t> groupsVec = { U("editors"), U("contributors") };
boost::optional<std::vector<utility::string_t>> mentionGroupIds = boost::optional<std::vector<utility::string_t>>(groupsVec);
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-user-456"));
boost::optional<utility::string_t> searchSection = boost::optional<utility::string_t>(U("discussion"));

api->searchUsers(tenantId, urlId, usernameStartsWith, mentionGroupIds, sso, searchSection)
    .then([](pplx::task<std::shared_ptr<SearchUsers_200_response>> task) {
        try {
            auto resp = task.get();
            auto respCopy = std::make_shared<SearchUsers_200_response>(*resp);
            (void)respCopy;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---