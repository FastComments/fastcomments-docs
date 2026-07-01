## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| createFeedPostParams | CreateFeedPostParams | כן |  |
| options | const CreateFeedPostPublicOptions& | כן |  |

## תגובה

מחזיר: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמת createFeedPostPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");

CreateFeedPostParams params;
params.title = U("Introducing Our New Feature");
params.content = U("We are excited to announce the release of our latest update.");
params.authorEmail = boost::optional<utility::string_t>(U("user@example.com"));
params.tags = boost::optional<std::vector<utility::string_t>>({U("announcement"), U("release")});

CreateFeedPostPublicOptions options;

api->createFeedPostPublic(tenantId, params, options)
    .then([](std::shared_ptr<CreateFeedPostResponse> resp) {
        if (resp) {
            std::wcout << U("Post created with ID: ") << resp->postId << std::endl;
        }
    });
[inline-code-end]