## 参数

| Name | Type | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| sso | string | 否 |  |

## 返回

返回: [`LockComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/LockComment_200_response.h)

## 示例

[inline-code-attrs-start title = 'unLockComment 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654");
utility::string_t broadcastId = U("brd-456");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
auto dummyResp = std::make_shared<LockComment_200_response>();
api->unLockComment(tenantId, commentId, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<LockComment_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            std::cout << "Comment unlocked successfully\n";
        } else {
            std::cout << "Unlock returned empty response\n";
        }
    } catch (const std::exception& e) {
        std::cerr << "Error unlocking comment: " << e.what() << '\n';
    }
});
[inline-code-end]

---