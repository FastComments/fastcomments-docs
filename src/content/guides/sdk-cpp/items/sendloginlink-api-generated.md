## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| redirectURL | string | No |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'sendLoginLink Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> redirect = boost::optional<utility::string_t>(U("https://app.example.com/welcome"));
auto fallback = std::make_shared<FlagCommentPublic_200_response>();
api->sendLoginLink(tenantId, userId, redirect).then([fallback](std::shared_ptr<FlagCommentPublic_200_response> resp){
    auto result = resp ? resp : fallback;
    std::cout << "sendLoginLink completed, response " << (result ? "received" : "none") << std::endl;
}).wait();
[inline-code-end]
