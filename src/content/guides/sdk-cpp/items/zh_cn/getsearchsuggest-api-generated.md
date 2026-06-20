## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| textSearch | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationSuggestResponse.h)

## 示例

[inline-code-attrs-start title = 'getSearchSuggest 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> textSearch = utility::string_t(U("preventing spam in comments"));
boost::optional<utility::string_t> sso = utility::string_t(U("user@example.com"));
api->getSearchSuggest(textSearch, sso)
    .then([](pplx::task<std::shared_ptr<ModerationSuggestResponse>> t) {
        try {
            auto resp = t.get();
            if (resp) {
                auto copy = std::make_shared<ModerationSuggestResponse>(*resp);
            }
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---