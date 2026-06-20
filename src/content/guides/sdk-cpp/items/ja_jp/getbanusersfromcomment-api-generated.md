## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| commentId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetBannedUsersFromCommentResponse.h)

## 例

[inline-code-attrs-start title = 'getBanUsersFromComment の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("comment-abc-123");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
std::shared_ptr<GetBannedUsersFromCommentResponse> result;
api->getBanUsersFromComment(commentId, sso).then([&result](pplx::task<std::shared_ptr<GetBannedUsersFromCommentResponse>> t){
    try {
        auto resp = t.get();
        if (resp) result = std::make_shared<GetBannedUsersFromCommentResponse>(*resp);
    } catch (const std::exception&) {
        result.reset();
    }
});
[inline-code-end]