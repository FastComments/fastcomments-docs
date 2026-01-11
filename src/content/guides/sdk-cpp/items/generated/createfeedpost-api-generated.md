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
utility::string_t tenantId = U("my-tenant-123");
CreateFeedPostParams createFeedPostParams;
createFeedPostParams.title = U("Product update: v2.1");
createFeedPostParams.content = U("We are releasing version 2.1 with performance and stability improvements.");
createFeedPostParams.authorEmail = U("author@example.com");
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-789"));
boost::optional<bool> isLive = boost::optional<bool>(true);
boost::optional<bool> doSpamCheck = boost::optional<bool>(false);
boost::optional<bool> skipDupCheck = boost::optional<bool>(false);

api->createFeedPost(tenantId, createFeedPostParams, broadcastId, isLive, doSpamCheck, skipDupCheck)
.then([](pplx::task<std::shared_ptr<CreateFeedPost_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto copy = std::make_shared<CreateFeedPost_200_response>(*resp);
            std::cout << "Feed post created" << std::endl;
        }
    } catch (const std::exception& e) {
        std::cerr << "CreateFeedPost failed: " << e.what() << std::endl;
    }
});
[inline-code-end]
