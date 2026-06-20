## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| updatableCommentParams | UpdatableCommentParams | Da |  |
| contextUserId | string | Ne |  |
| doSpamCheck | bool | Ne |  |
| isLive | bool | Ne |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer updateComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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