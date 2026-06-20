## 参数

| 名称 | 类型 | 必需 | 说明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| isFlagged | bool | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 示例

[inline-code-attrs-start title = 'flagCommentPublic 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
bool isFlagged = true;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->flagCommentPublic(tenantId, commentId, isFlagged, sso)
    .then([](std::shared_ptr<APIEmptyResponse> resp){
        auto result = resp ? resp : std::make_shared<APIEmptyResponse>();
        std::cout << "Flag update completed\n";
    });
[inline-code-end]

---