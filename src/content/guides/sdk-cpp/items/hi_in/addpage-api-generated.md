## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createAPIPageData | CreateAPIPageData | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/AddPageAPIResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'addPage उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto createData = CreateAPIPageData{};
createData.title = utility::string_t(U("Welcome Page"));
createData.url = utility::string_t(U("https://example.com/welcome"));
createData.description = boost::optional<utility::string_t>(utility::string_t(U("Landing page for new users")));

api->addPage(utility::string_t(U("my-tenant-123")), createData)
    .then([](std::shared_ptr<AddPageAPIResponse> response) {
        if (response && response->success) {
            // सफल जोड़ को हैंडल करें
        } else {
            // त्रुटि को हैंडल करें
        }
    });
[inline-code-end]