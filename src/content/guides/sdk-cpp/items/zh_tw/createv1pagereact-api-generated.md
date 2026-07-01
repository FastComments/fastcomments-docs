## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| title | string | 否 |  |

## 回應

返回：[`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## 範例

[inline-code-attrs-start title = 'createV1PageReact 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto urlId = utility::string_t(U("https://example.com/articles/789"));
boost::optional<utility::string_t> title = utility::string_t(U("Understanding Async C++"));
api->createV1PageReact(tenantId, urlId, title).then([](pplx::task<std::shared_ptr<CreateV1PageReact>> task) {
    try {
        auto response = task.get();
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---