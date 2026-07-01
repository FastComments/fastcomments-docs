## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| options | const GetUserReactsPublicOptions& | 是 |  |

## 响应

返回: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UserReactsResponse.h)

## 示例

[inline-code-attrs-start title = 'getUserReactsPublic 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = GetUserReactsPublicOptions{};
options.userId = boost::optional<utility::string_t>(U("user@example.com"));
options.limit = boost::optional<int>(50);
api->getUserReactsPublic(U("my-tenant-123"), options).then([](pplx::task<std::shared_ptr<UserReactsResponse>> t){
    auto response = t.get();
});
[inline-code-end]