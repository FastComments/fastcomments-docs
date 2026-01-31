## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`PinComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PinComment_200_response.h)

## Example

[inline-code-attrs-start title = 'pinComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456");
utility::string_t broadcastId = U("broadcast-789");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user-jwt-token-abc.def.ghi"));

api->pinComment(tenantId, commentId, broadcastId, sso)
.then([](std::shared_ptr<PinComment_200_response> resp){
    if (resp) {
        auto copy = std::make_shared<PinComment_200_response>(*resp);
    }
});
[inline-code-end]
