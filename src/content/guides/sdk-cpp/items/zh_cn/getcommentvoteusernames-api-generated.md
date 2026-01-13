## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| dir | int32_t | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`GetCommentVoteUserNames_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentVoteUserNames_200_response.h)

## 示例

[inline-code-attrs-start title = 'getCommentVoteUserNames 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456");
int32_t dir = 1;
boost::optional<utility::string_t> sso{ utility::string_t(U("user@example.com")) };
api->getCommentVoteUserNames(tenantId, commentId, dir, sso)
    .then([](pplx::task<std::shared_ptr<GetCommentVoteUserNames_200_response>> t) {
        try {
            auto resp = t.get();
            if (!resp) resp = std::make_shared<GetCommentVoteUserNames_200_response>();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---