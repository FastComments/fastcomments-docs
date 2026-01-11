## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| usernameStartsWith | string | Yes |  |
| mentionGroupIds | vector<string | No |  |
| sso | string | No |  |

## Response

Returns: [`SearchUsers_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SearchUsers_200_response.h)

## Example

[inline-code-attrs-start title = 'searchUsers Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t urlId = utility::conversions::to_string_t("article-456");
utility::string_t usernameStartsWith = utility::conversions::to_string_t("john");
boost::optional<std::vector<utility::string_t>> mentionGroupIds = std::vector<utility::string_t>{
    utility::conversions::to_string_t("editors"),
    utility::conversions::to_string_t("staff")
};
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("google");
api->searchUsers(tenantId, urlId, usernameStartsWith, mentionGroupIds, sso)
.then([](pplx::task<std::shared_ptr<SearchUsers_200_response>> task){
    try {
        auto resp = task.get();
        (void)resp;
    } catch (const std::exception& e) {
        auto fallback = std::make_shared<SearchUsers_200_response>();
        (void)fallback; (void)e;
    }
});
[inline-code-end]
