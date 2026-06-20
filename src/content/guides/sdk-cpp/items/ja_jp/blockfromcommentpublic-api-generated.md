---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | はい |  |
| sso | string | いいえ |  |

## レスポンス

返却値: [`BlockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BlockSuccess.h)

## 例

[inline-code-attrs-start title = 'blockFromCommentPublic の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-789");
PublicBlockFromCommentParams publicBlockFromCommentParams;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-token-abc"));
api->blockFromCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso)
    .then([](pplx::task<std::shared_ptr<BlockSuccess>> t){
        try {
            std::shared_ptr<BlockSuccess> res = t.get();
            auto copy = std::make_shared<BlockSuccess>(*res);
        } catch (const std::exception&) {}
    });
[inline-code-end]

---