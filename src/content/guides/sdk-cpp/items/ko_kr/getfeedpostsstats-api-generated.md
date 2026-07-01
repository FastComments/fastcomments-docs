## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| postIds | vector<string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FeedPostsStatsResponse.h)

## 예제

[inline-code-attrs-start title = 'getFeedPostsStats 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
std::vector<utility::string_t> postIds = {
    utility::conversions::to_string_t("post-001"),
    utility::conversions::to_string_t("post-002")
};
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("user@example.com");

api->getFeedPostsStats(tenantId, postIds, sso)
    .then([](std::shared_ptr<FeedPostsStatsResponse> response) {
        (void)response;
    });
[inline-code-end]