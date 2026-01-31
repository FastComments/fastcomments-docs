## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createFeedPostParams | CreateFeedPostParams | Yes |  |
| broadcastId | string | No |  |
| isLive | bool | No |  |
| doSpamCheck | bool | No |  |
| skipDupCheck | bool | No |  |

## Response

Returns: [`CreateFeedPost_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPost_200_response.h)

## Example

[inline-code-attrs-start title = 'createFeedPost Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
CreateFeedPostParams createFeedPostParams;
createFeedPostParams.setTitle(utility::string_t(U("New Feature Released")));
createFeedPostParams.setBody(utility::string_t(U("We're excited to launch the new editor feature.")));
createFeedPostParams.setAuthorEmail(utility::string_t(U("product-team@example.com")));

boost::optional<utility::string_t> broadcastId = utility::string_t(U("broadcast-789"));
boost::optional<bool> isLive = true;
boost::optional<bool> doSpamCheck = false;
boost::optional<bool> skipDupCheck = boost::none;

api->createFeedPost(utility::string_t(U("my-tenant-123")), createFeedPostParams, broadcastId, isLive, doSpamCheck, skipDupCheck)
.then([](std::shared_ptr<CreateFeedPost_200_response> resp){
    if(!resp) return;
    auto processed = std::make_shared<CreateFeedPost_200_response>(*resp);
});
[inline-code-end]
