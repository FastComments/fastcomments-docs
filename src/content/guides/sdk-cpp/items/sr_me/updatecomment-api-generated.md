## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updatableCommentParams | UpdatableCommentParams | Да |  |
| contextUserId | string | Не |  |
| doSpamCheck | bool | Не |  |
| isLive | bool | Не |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Пример

[inline-code-attrs-start title = 'updateComment Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654321");
auto params = std::make_shared<UpdatableCommentParams>();
params->body = U("Updated comment to clarify the timeline and remove profanity");
boost::optional<utility::string_t> contextUserId = boost::optional<utility::string_t>(U("moderator@myapp.com"));
boost::optional<bool> doSpamCheck = boost::optional<bool>(true);
boost::optional<bool> isLive = boost::optional<bool>(true);
api->updateComment(tenantId, commentId, *params, contextUserId, doSpamCheck, isLive)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t){
    try {
        auto resp = t.get();
        (void)resp;
    } catch(const std::exception&) {
    }
});
[inline-code-end]

---