## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| options | const PostUnFlagCommentOptions& | Да |  |

## Response

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример postUnFlagComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
PostUnFlagCommentOptions opts;
opts.notifyUser = boost::optional<bool>(true);
api->postUnFlagComment(tenantId, commentId, opts)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {
        // здесь можно выполнить обработку
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception&) {}
    });
[inline-code-end]