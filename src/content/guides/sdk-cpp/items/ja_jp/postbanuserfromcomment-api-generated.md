## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| options | const PostBanUserFromCommentOptions& | はい |  |

## レスポンス

戻り値: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BanUserFromCommentResult.h)

## 例

[inline-code-attrs-start title = 'postBanUserFromComment 例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456789");
PostBanUserFromCommentOptions options;
options.reason = boost::optional<utility::string_t>(U("spam"));
options.durationDays = boost::optional<int>(30);

api->postBanUserFromComment(tenantId, commentId, options)
    .then([](std::shared_ptr<BanUserFromCommentResult> result) {
        // 結果の処理
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception& e) { /* エラー処理 */ }
    });
[inline-code-end]