## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| usernameStartsWith | string | 아니요 |  |
| mentionGroupIds | vector<string | 아니요 |  |
| sso | string | 아니요 |  |
| searchSection | string | 아니요 |  |

## 응답

반환: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SearchUsersResult.h)

## 예제

[inline-code-attrs-start title = 'searchUsers 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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