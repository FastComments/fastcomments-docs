## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| postId | string | हाँ |  |
| reactBodyParams | ReactBodyParams | हाँ |  |
| options | const ReactFeedPostPublicOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/ReactFeedPostResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'reactFeedPostPublic उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto postId = utility::conversions::to_string_t("post-987");
ReactBodyParams reactBody;
reactBody.reaction = utility::conversions::to_string_t("love");
reactBody.userId = utility::conversions::to_string_t("user@example.com");
ReactFeedPostPublicOptions options;
options.metadata = boost::optional<utility::string_t>(utility::conversions::to_string_t("mobile"));
api->reactFeedPostPublic(tenantId, postId, reactBody, options)
    .then([](std::shared_ptr<ReactFeedPostResponse> resp) {
    });
[inline-code-end]