## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| options | const GetUserBadgesOptions& | 是 |  |

## 响应

返回：[`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetUserBadgesResponse.h)

## 示例

[inline-code-attrs-start title = 'getUserBadges 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GetUserBadgesOptions opts;
opts.userId = boost::make_optional(U("user@example.com"));
opts.includeExpired = boost::make_optional(false);

api->getUserBadges(U("my-tenant-123"), opts)
   .then([](pplx::task<std::shared_ptr<APIGetUserBadgesResponse>> t) {
       try {
           auto response = t.get();
       } catch (const std::exception&) {
       }
   });
[inline-code-end]

---