## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Yes |  |
| sso | string | No |  |

## 响应

返回：[`BlockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BlockSuccess.h)

## 示例

[inline-code-attrs-start title = 'blockFromCommentPublic 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("comment-987654");
PublicBlockFromCommentParams blockParams;
blockParams.reason = utility::conversions::to_string_t("spam");
blockParams.durationHours = 24;
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("sso-token-abc123");
api->blockFromCommentPublic(tenantId, commentId, blockParams, sso)
    .then([](std::shared_ptr<BlockSuccess> result){
        auto successCopy = std::make_shared<BlockSuccess>(*result);
    });
[inline-code-end]

---