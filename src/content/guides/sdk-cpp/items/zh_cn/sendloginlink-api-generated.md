## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| redirectURL | string | 否 |  |

## 响应

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 示例

[inline-code-attrs-start title = 'sendLoginLink 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->sendLoginLink(
    U("my-tenant-123"),
    U("user@example.com"),
    boost::make_optional(U("https://myapp.com/auth/callback"))
).then([](std::shared_ptr<APIEmptyResponse> resp) {
});
[inline-code-end]

---