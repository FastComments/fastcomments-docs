## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetSearchSitesOptions& | Yes |  |

## 回應

返回：[`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationSiteSearchResponse.h)

## 範例

[inline-code-attrs-start title = 'getSearchSites 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
GetSearchSitesOptions options;
options.query = boost::make_optional(utility::string_t(U("example query")));
options.limit = boost::make_optional(25);

api->getSearchSites(tenantId, options)
   .then([](pplx::task<std::shared_ptr<ModerationSiteSearchResponse>> t) {
       try {
           auto response = t.get();
           auto respPtr = std::make_shared<ModerationSiteSearchResponse>(*response);
           for (const auto& site : respPtr->sites) {
               // 處理邏輯
           }
       } catch (const std::exception&) {
           // 錯誤處理
       }
   });
[inline-code-end]