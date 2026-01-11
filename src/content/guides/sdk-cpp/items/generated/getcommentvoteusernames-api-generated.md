## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| dir | int32_t | Yes |  |
| sso | string | No |  |

## Response

Returns: [`GetCommentVoteUserNames_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentVoteUserNames_200_response.h)

## Example

[inline-code-attrs-start title = 'getCommentVoteUserNames Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t commentId = utility::conversions::to_string_t("cmt-987654321");
int32_t dir = 1;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(utility::conversions::to_string_t("user@example.com"));
api->getCommentVoteUserNames(tenantId, commentId, dir, sso)
.then([](pplx::task<std::shared_ptr<GetCommentVoteUserNames_200_response>> t) {
    try {
        auto resp = t.get();
        if (resp) {
            auto result = std::make_shared<GetCommentVoteUserNames_200_response>(*resp);
        }
    } catch (const std::exception& e) {
        auto fallback = std::make_shared<GetCommentVoteUserNames_200_response>();
    }
});
[inline-code-end]
