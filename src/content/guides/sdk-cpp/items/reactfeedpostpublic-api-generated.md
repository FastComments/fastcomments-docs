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
auto tenantId = utility::string_t(U("my-tenant-123"));
auto postId = utility::string_t(U("post-9876"));
auto reactBodyPtr = std::make_shared<ReactBodyParams>();
reactBodyPtr->reaction = utility::string_t(U("like"));
boost::optional<bool> isUndo = boost::optional<bool>(false);
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(utility::string_t(U("broadcast-42")));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(utility::string_t(U("user@example.com")));
api->reactFeedPostPublic(tenantId, postId, *reactBodyPtr, isUndo, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<ReactFeedPostPublic_200_response>> task){
    try {
        auto resp = task.get();
        (void)resp;
    } catch (const std::exception&) {}
});
[inline-code-end]
