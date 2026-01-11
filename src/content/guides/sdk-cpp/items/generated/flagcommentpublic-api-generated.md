## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| isFlagged | bool | Yes |  |
| sso | string | No |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'flagCommentPublic Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> sso = utility::string_t(U("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.token"));
auto task = api->flagCommentPublic(utility::string_t(U("my-tenant-123")), utility::string_t(U("cmt-456")), true, sso)
    .then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) {
        try {
            auto resp = t.get();
            auto result = std::make_shared<FlagCommentPublic_200_response>(*resp);
            return result;
        } catch (const std::exception&) {
            return std::make_shared<FlagCommentPublic_200_response>();
        }
    });
[inline-code-end]
