## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetSearchPagesOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/ModerationPageSearchResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getSearchPages उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GetSearchPagesOptions options;
options.pageNumber = boost::optional<int>(1);
options.pageSize = boost::optional<int>(50);
options.searchTerm = boost::optional<utility::string_t>(U("spam"));

api->getSearchPages(U("my-tenant-123"), options)
    .then([](pplx::task<std::shared_ptr<ModerationPageSearchResponse>> task) {
        try {
            auto response = task.get();
            // response का उपयोग करें
        } catch (const std::exception& e) {
            // त्रुटि को संभालें
        }
    });
[inline-code-end]