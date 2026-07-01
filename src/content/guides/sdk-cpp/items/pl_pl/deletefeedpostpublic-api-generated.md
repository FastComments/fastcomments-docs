## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| postId | string | Tak |  |
| options | const DeleteFeedPostPublicOptions& | Tak |  |

## Odpowiedź

Zwraca: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteFeedPostPublicResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteFeedPostPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto opts = std::make_shared<DeleteFeedPostPublicOptions>();
opts->reason = boost::optional<utility::string_t>(U("Inappropriate content"));
api->deleteFeedPostPublic(U("my-tenant-123"), U("post-789"), *opts)
    .then([](std::shared_ptr<DeleteFeedPostPublicResponse> resp) {
        if (resp && resp->isSuccess()) {
            // success logic
        }
    });
[inline-code-end]