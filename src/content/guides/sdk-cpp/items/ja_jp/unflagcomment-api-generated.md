## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| options | const UnFlagCommentOptions& | Yes |  |

## 応答

返却: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentResponse.h)

## 例

[inline-code-attrs-start title = 'unFlagComment の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = UnFlagCommentOptions{};
options.reason = boost::optional<utility::string_t>(U("Resolved by moderator"));
api->unFlagComment(U("my-tenant-123"), U("comment-456"), options)
    .then([](std::shared_ptr<FlagCommentResponse> response) {
        if (response) {
            auto status = response->status;
            // 必要に応じてステータスを処理する
        }
    })
    .then([](pplx::task<void> previous) {
        try {
            previous.get();
        } catch (const std::exception& e) {
            // エラーを処理する
        }
    });
[inline-code-end]