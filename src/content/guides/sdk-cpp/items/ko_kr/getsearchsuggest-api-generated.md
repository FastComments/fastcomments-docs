## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| options | const GetSearchSuggestOptions& | 예 |  |

## 응답

반환: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationSuggestResponse.h)

## 예제

[inline-code-attrs-start title = 'getSearchSuggest 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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