## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| postId | string | हाँ |  |
| options | const DeleteFeedPostPublicOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/DeleteFeedPostPublicResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'deleteFeedPostPublic उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto opts = std::make_shared<DeleteFeedPostPublicOptions>();
opts->reason = boost::optional<utility::string_t>(U("Inappropriate content"));
api->deleteFeedPostPublic(U("my-tenant-123"), U("post-789"), *opts)
    .then([](std::shared_ptr<DeleteFeedPostPublicResponse> resp) {
        if (resp && resp->isSuccess()) {
            // सफलता का लॉजिक
        }
    });
[inline-code-end]