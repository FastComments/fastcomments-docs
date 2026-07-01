## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| options | const GetSearchSuggestOptions& | Ja |  |

## Respons

Returns: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationSuggestResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getSearchSuggest Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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