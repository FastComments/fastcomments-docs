---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| textSearch | string | 否 |  |
| byIPFromComment | string | 否 |  |
| filter | string | 否 |  |
| searchFilters | string | 否 |  |
| demo | bool | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPICountCommentsResponse.h)

## 示例

[inline-code-attrs-start title = 'getCount 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t text = U("abusive language");
utility::string_t ip = U("203.0.113.45");
utility::string_t filter = U("status:flagged");
utility::string_t searchFilters = U("platform:mobile");
utility::string_t sso = U("admin@my-tenant-123.com");
auto textOpt = boost::optional<utility::string_t>(text);
auto ipOpt = boost::optional<utility::string_t>(ip);
auto filterOpt = boost::optional<utility::string_t>(filter);
auto searchFiltersOpt = boost::optional<utility::string_t>(searchFilters);
auto demoOpt = boost::optional<bool>(true);
auto ssoOpt = boost::optional<utility::string_t>(sso);
api->getCount(textOpt, ipOpt, filterOpt, searchFiltersOpt, demoOpt, ssoOpt)
.then([](pplx::task<std::shared_ptr<ModerationAPICountCommentsResponse>> t){
    try {
        auto resp = t.get();
        auto finalResp = resp ? resp : std::make_shared<ModerationAPICountCommentsResponse>();
        (void)finalResp;
    } catch (...) {}
});
[inline-code-end]

---