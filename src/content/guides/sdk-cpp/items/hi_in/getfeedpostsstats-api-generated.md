## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| postIds | vector<string | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/FeedPostsStatsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getFeedPostsStats उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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