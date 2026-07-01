## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| options | const DeleteFeedPostPublicOptions& | Yes |  |

## Odgovor

Returns: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteFeedPostPublicResponse.h)

## Primjer

[inline-code-attrs-start title = 'deleteFeedPostPublic Primjer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto opts = std::make_shared<DeleteFeedPostPublicOptions>();
opts->reason = boost::optional<utility::string_t>(U("Inappropriate content"));
api->deleteFeedPostPublic(U("my-tenant-123"), U("post-789"), *opts)
    .then([](std::shared_ptr<DeleteFeedPostPublicResponse> resp) {
        if (resp && resp->isSuccess()) {
            // logika uspjeha
        }
    });
[inline-code-end]