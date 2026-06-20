---
## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| value | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回： [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationSiteSearchResponse.h)

## 示例

[inline-code-attrs-start title = 'getSearchSites 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> valueOpt = boost::optional<utility::string_t>(U("my-tenant-123"));
boost::optional<utility::string_t> ssoOpt = boost::optional<utility::string_t>(U("user@example.com"));
api->getSearchSites(valueOpt, ssoOpt)
    .then([](std::shared_ptr<ModerationSiteSearchResponse> resp){
        auto response = resp ? resp : std::make_shared<ModerationSiteSearchResponse>();
        (void)response;
    })
    .wait();
[inline-code-end]

---