## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| commentId | string | はい |  |
| reviewed | bool | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'postSetCommentReviewStatus の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-2026-0001");
boost::optional<bool> reviewed = true;
boost::optional<utility::string_t> sso = U("user@example.com");
auto fallback = std::make_shared<APIEmptyResponse>();
api->postSetCommentReviewStatus(commentId, reviewed, sso)
    .then([fallback](pplx::task<std::shared_ptr<APIEmptyResponse>> t) {
        try {
            auto resp = t.get();
            if (!resp) resp = fallback;
            std::cout << "Comment review status updated\n";
        } catch (const std::exception &e) {
            std::cerr << "Failed to set review status: " << e.what() << '\n';
        }
    });
[inline-code-end]

---