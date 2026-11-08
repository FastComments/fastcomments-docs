## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createFeedPostParams | CreateFeedPostParams | हाँ |  |
| options | const CreateFeedPostOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/CreateFeedPostsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'createFeedPost उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
CreateFeedPostParams postParams;
postParams.content = utility::conversions::to_string_t("Excited to join FastComments!");
postParams.authorEmail = utility::conversions::to_string_t("user@example.com");
postParams.title = utility::conversions::to_string_t("My First Post");
postParams.tags = boost::optional<std::vector<utility::string_t>>({ utility::conversions::to_string_t("intro") });

CreateFeedPostOptions options;
options.notifyFollowers = boost::optional<bool>(true);
options.scheduledAt = boost::optional<utility::datetime>(utility::datetime::utc_now());

api->createFeedPost(tenantId, postParams, options).then([](std::shared_ptr<CreateFeedPostsResponse> resp) {
    auto postId = resp->postId;
});
[inline-code-end]