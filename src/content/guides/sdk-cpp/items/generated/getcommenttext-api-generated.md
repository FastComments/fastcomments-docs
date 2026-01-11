## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`GetCommentText_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentText_200_response.h)

## Example

[inline-code-attrs-start title = 'getCommentText Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654321");
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(U("editkey-456"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-token-abc123"));
api->getCommentText(tenantId, commentId, editKey, sso)
.then([](pplx::task<std::shared_ptr<GetCommentText_200_response>> t){
    try {
        auto resp = t.get();
        auto safeResp = resp ? resp : std::make_shared<GetCommentText_200_response>();
    } catch (const std::exception &e) {
        auto fallback = std::make_shared<GetCommentText_200_response>();
    }
});
[inline-code-end]
