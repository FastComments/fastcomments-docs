## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`FlagComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagComment_200_response.h)

## Example

[inline-code-attrs-start title = 'flagComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t commentId = utility::conversions::to_string_t("cmt-456789");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(utility::conversions::to_string_t("user@example.com"));
boost::optional<utility::string_t> anonUserId = boost::optional<utility::string_t>(utility::conversions::to_string_t("anon-uuid-789"));
auto t = api->flagComment(tenantId, commentId, userId, anonUserId)
.then([](pplx::task<std::shared_ptr<FlagComment_200_response>> task){
    try {
        auto resp = task.get();
        auto marker = std::make_shared<utility::string_t>(utility::conversions::to_string_t("comment_flagged"));
        (void)resp;
        (void)marker;
    } catch (const std::exception&) {
    }
});
[inline-code-end]
