## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 响应

Returns: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetVotesResponse.h)

## 示例

[inline-code-attrs-start title = 'getVotes 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto urlId = utility::string_t(U("article-456"));
boost::optional<utility::string_t> extraHeader = boost::none;

api->getVotes(tenantId, urlId).then([=](pplx::task<std::shared_ptr<GetVotesResponse>> task) {
    try {
        auto original = task.get();
        auto response = std::make_shared<GetVotesResponse>(*original);
    } catch (...) {
        // 错误处理
    }
});
[inline-code-end]