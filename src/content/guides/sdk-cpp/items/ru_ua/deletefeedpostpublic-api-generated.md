## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|-------------|------|
| tenantId | string | Так |  |
| postId | string | Так |  |
| options | const DeleteFeedPostPublicOptions& | Так |  |

## Відповідь

Повертає: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteFeedPostPublicResponse.h)

## Приклад

[inline-code-attrs-start title = 'deleteFeedPostPublic Приклад'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto opts = std::make_shared<DeleteFeedPostPublicOptions>();
opts->reason = boost::optional<utility::string_t>(U("Inappropriate content"));
api->deleteFeedPostPublic(U("my-tenant-123"), U("post-789"), *opts)
    .then([](std::shared_ptr<DeleteFeedPostPublicResponse> resp) {
        if (resp && resp->isSuccess()) {
            // логіка успіху
        }
    });
[inline-code-end]