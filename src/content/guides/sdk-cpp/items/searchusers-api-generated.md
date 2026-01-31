## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| usernameStartsWith | string | No |  |
| mentionGroupIds | vector<string | No |  |
| sso | string | No |  |

## Response

Returns: [`SearchUsers_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SearchUsers_200_response.h)

## Example

[inline-code-attrs-start title = 'searchUsers Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("article-456");
boost::optional<utility::string_t> usernameStartsWith = boost::optional<utility::string_t>(U("alice"));
boost::optional<std::vector<utility::string_t>> mentionGroupIds = boost::optional<std::vector<utility::string_t>>({ U("engineering"), U("moderators") });
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("saml-provider"));

api->searchUsers(tenantId, urlId, usernameStartsWith, mentionGroupIds, sso)
    .then([](std::shared_ptr<SearchUsers_200_response> resp) {
        if (resp)
            std::cout << "Users retrieved successfully" << std::endl;
        else
            std::cout << "No users found" << std::endl;
    });
[inline-code-end]
