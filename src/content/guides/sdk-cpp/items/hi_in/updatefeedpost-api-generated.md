## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| feedPost | FeedPost | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'updateFeedPost उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U"my-tenant-123";
utility::string_t postId = U"post-456";

FeedPost feedPost;
feedPost.title = U"Breaking News";
feedPost.content = U"Details of the update go here.";
feedPost.imageUrl = boost::optional<utility::string_t>(U"https://example.com/image.jpg");

api->updateFeedPost(tenantId, postId, feedPost)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task) {
        try {
            auto response = task.get();
        } catch (const std::exception& ex) {
        }
    });
[inline-code-end]