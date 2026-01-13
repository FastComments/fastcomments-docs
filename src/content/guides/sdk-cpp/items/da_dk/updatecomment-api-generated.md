## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updatableCommentParams | UpdatableCommentParams | Yes |  |
| contextUserId | string | No |  |
| doSpamCheck | bool | No |  |
| isLive | bool | No |  |

## Respons

Returnerer: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på updateComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto commentId = utility::string_t(U("comment-456"));
auto body = std::make_shared<UpdatableCommentParams>();
body->content = utility::string_t(U("Updated comment text: fixed a typo and clarified meaning."));
boost::optional<utility::string_t> contextUserId(utility::string_t(U("user@example.com")));
boost::optional<bool> doSpamCheck(true);
boost::optional<bool> isLive(false);
api->updateComment(tenantId, commentId, body, contextUserId, doSpamCheck, isLive)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t){
    try {
        auto result = t.get();
        (void)result;
    } catch (...) {}
});
[inline-code-end]