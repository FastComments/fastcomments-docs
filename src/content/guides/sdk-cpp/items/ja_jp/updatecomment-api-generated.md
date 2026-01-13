## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updatableCommentParams | UpdatableCommentParams | はい |  |
| contextUserId | string | いいえ |  |
| doSpamCheck | bool | いいえ |  |
| isLive | bool | いいえ |  |

## レスポンス

戻り値: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 例

[inline-code-attrs-start title = 'updateComment の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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

---