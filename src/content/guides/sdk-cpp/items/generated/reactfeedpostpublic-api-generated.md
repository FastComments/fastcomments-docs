## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| reactBodyParams | ReactBodyParams | Yes |  |
| isUndo | bool | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`ReactFeedPostPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ReactFeedPostPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'reactFeedPostPublic Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("post-9876");
auto reactBodyPtr = std::make_shared<ReactBodyParams>();
boost::optional<bool> isUndo = false;
boost::optional<utility::string_t> broadcastId = U("broadcast-2025");
boost::optional<utility::string_t> sso = U("user@example.com");
api->reactFeedPostPublic(tenantId, postId, *reactBodyPtr, isUndo, broadcastId, sso)
.then([](std::shared_ptr<ReactFeedPostPublic_200_response> resp){
    (void)resp;
});
[inline-code-end]
