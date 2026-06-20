## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| broadcastId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'unLockComment の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
utility::string_t broadcastId = U("bcast-987");
boost::optional<utility::string_t> sso = utility::string_t(U("user@example.com"));
api->unLockComment(tenantId, commentId, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            std::cout << "Comment unlocked successfully\n";
        } else {
            std::cout << "No response body\n";
        }
    } catch (const std::exception& e) {
        auto emptyResp = std::make_shared<APIEmptyResponse>();
        std::cout << "Error unlocking comment: " << e.what() << "\n";
    }
});
[inline-code-end]

---