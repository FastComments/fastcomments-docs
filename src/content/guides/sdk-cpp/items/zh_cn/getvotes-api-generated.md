## 参数

| 名称 | 类型 | 是否必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 响应

返回：[`GetVotes_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetVotes_200_response.h)

## 示例

[inline-code-attrs-start title = 'getVotes 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> urlId = utility::string_t(U("/posts/2025/new-features"));
auto task = api->getVotes(tenantId.value(), urlId.value()).then([](std::shared_ptr<GetVotes_200_response> resp){
    if (resp) {
        auto copy = std::make_shared<GetVotes_200_response>(*resp);
        std::cout << "Fetched votes successfully\n";
    } else {
        std::cout << "No votes response\n";
    }
});
task.wait();
[inline-code-end]

---