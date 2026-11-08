## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetSearchSuggestOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/ModerationSuggestResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getSearchSuggest उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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