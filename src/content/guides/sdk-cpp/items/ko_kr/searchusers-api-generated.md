## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| usernameStartsWith | string | 아니오 |  |
| mentionGroupIds | vector<string | 아니오 |  |
| sso | string | 아니오 |  |
| searchSection | string | 아니오 |  |

## 응답

반환: [`SearchUsers_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SearchUsers_200_response.h)

## 예제

[inline-code-attrs-start title = 'searchUsers 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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