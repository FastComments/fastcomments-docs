## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| editKey | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`DeleteCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteCommentPublic_200_response.h)

## 示例

[inline-code-attrs-start title = 'deleteCommentPublic 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId(U("my-tenant-123"));
utility::string_t commentId(U("cmt-456"));
utility::string_t broadcastId(U("brd-789"));
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(utility::string_t(U("editkey-abc123")));
boost::optional<utility::string_t> sso; 

api->deleteCommentPublic(tenantId, commentId, broadcastId, editKey, sso)
.then([](pplx::task<std::shared_ptr<DeleteCommentPublic_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto copied = std::make_shared<DeleteCommentPublic_200_response>(*resp);
            (void)copied;
        }
    } catch (const std::exception&) {
        auto fallback = std::make_shared<DeleteCommentPublic_200_response>();
        (void)fallback;
    }
});
[inline-code-end]

---