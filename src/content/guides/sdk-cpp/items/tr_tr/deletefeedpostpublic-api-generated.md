## Parametreler

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|----------|
| tenantId | string | Evet |  |
| postId | string | Evet |  |
| options | const DeleteFeedPostPublicOptions& | Evet |  |

## Yanıt

Döndürür: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteFeedPostPublicResponse.h)

## Örnek

[inline-code-attrs-start title = 'deleteFeedPostPublic Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto opts = std::make_shared<DeleteFeedPostPublicOptions>();
opts->reason = boost::optional<utility::string_t>(U("Inappropriate content"));
api->deleteFeedPostPublic(U("my-tenant-123"), U("post-789"), *opts)
    .then([](std::shared_ptr<DeleteFeedPostPublicResponse> resp) {
        if (resp && resp->isSuccess()) {
            // başarı mantığı
        }
    });
[inline-code-end]

---