## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 示例

[inline-code-attrs-start title = 'unLockComment 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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