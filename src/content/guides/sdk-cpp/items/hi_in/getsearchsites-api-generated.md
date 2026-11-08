## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetSearchSitesOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/ModerationSiteSearchResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getSearchSites उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
               // प्रोसेसिंग लॉजिक
           }
       } catch (const std::exception&) {
           // त्रुटि हैंडलिंग
       }
   });
[inline-code-end]