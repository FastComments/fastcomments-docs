## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/DeletePageAPIResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'deletePage उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto pageId   = utility::conversions::to_string_t("page-456");

api->deletePage(tenantId, pageId)
   .then([](pplx::task<std::shared_ptr<DeletePageAPIResponse>> t) {
       try {
           auto response = t.get();
           // आवश्यकतानुसार प्रतिक्रिया को प्रोसेस करें
       } catch (const std::exception& ex) {
           // त्रुटि को संभालें
       }
   });
[inline-code-end]