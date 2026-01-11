## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createFeedPostParams | CreateFeedPostParams | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`CreateFeedPostPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'createFeedPostPublic Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateFeedPostParams feedParams = CreateFeedPostParams();
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-456"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("alice@example.com"));
api->createFeedPostPublic(tenantId, feedParams, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<CreateFeedPostPublic_200_response>> t){
    try {
        auto resp = t.get();
        auto respCopy = std::make_shared<CreateFeedPostPublic_200_response>(*resp);
        (void)respCopy;
    } catch (...) {}
});
[inline-code-end]
