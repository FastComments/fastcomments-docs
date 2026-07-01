## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| postId | string | Da |  |
| updateFeedPostParams | UpdateFeedPostParams | Da |  |
| options | const UpdateFeedPostPublicOptions& | Da |  |

## Odgovor

Vraća: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostResponse.h)

## Primjer

[inline-code-attrs-start title = 'Primjer updateFeedPostPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("post-456");
UpdateFeedPostParams params;
params.title = boost::optional<utility::string_t>(U("Updated Title"));
params.body = boost::optional<utility::string_t>(U("Updated content of the post."));
UpdateFeedPostPublicOptions options;
options.notifyFollowers = boost::optional<bool>(true);
api->updateFeedPostPublic(tenantId, postId, params, options)
    .then([](std::shared_ptr<CreateFeedPostResponse> resp) {
        auto result = resp;
    });
[inline-code-end]