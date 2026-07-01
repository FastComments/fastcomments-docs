## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| options | const GetSearchSuggestOptions& | 是 |  |

## 回應

返回: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationSuggestResponse.h)

## 範例

[inline-code-attrs-start title = 'getSearchSuggest 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetSearchSuggestOptions opts;
opts.query = U("search term");
opts.limit = boost::optional<int>(5);
opts.includeInactive = boost::optional<bool>(false);
api->getSearchSuggest(tenantId, opts).then([](pplx::task<std::shared_ptr<ModerationSuggestResponse>> t){
    try{
        auto resp = t.get();
    }catch(const std::exception&){ }
}).wait();
[inline-code-end]

---