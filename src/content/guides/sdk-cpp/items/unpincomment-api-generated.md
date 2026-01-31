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

[inline-code-attrs-start title = 'unPinComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("c0a801f4-8f32-4b2a-9e2f-7d9b4f1a2e3c");
utility::string_t broadcastId = U("broadcast-456");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(utility::string_t(U("sso-token-abc123")));
api->unPinComment(tenantId, commentId, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<PinComment_200_response>> t){
    try {
        auto resp = t.get();
        if(!resp) resp = std::make_shared<PinComment_200_response>();
    } catch(const std::exception&) {
    }
});
[inline-code-end]
