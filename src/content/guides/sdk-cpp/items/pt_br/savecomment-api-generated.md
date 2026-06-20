## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| createCommentParams | CreateCommentParams | Sim |  |
| isLive | bool | Não |  |
| doSpamCheck | bool | Não |  |
| sendEmails | bool | Não |  |
| populateNotifications | bool | Não |  |

## Resposta

Retorna: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APISaveCommentResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de saveComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateCommentParams createParams;
createParams.threadId = utility::string_t(U("thread-789"));
createParams.body = utility::string_t(U("This is a test comment posted via SDK."));
createParams.authorEmail = utility::string_t(U("user@example.com"));
createParams.authorName = utility::string_t(U("Jane Developer"));
boost::optional<bool> isLive(true);
boost::optional<bool> doSpamCheck(false);
boost::optional<bool> sendEmails(true);
boost::optional<bool> populateNotifications(true);
api->saveComment(tenantId, createParams, isLive, doSpamCheck, sendEmails, populateNotifications)
.then([](pplx::task<std::shared_ptr<APISaveCommentResponse>> t){
    try {
        auto resp = t.get();
        auto marker = std::make_shared<bool>(true);
        (void)resp;
        (void)marker;
    } catch (const std::exception&) {}
});
[inline-code-end]