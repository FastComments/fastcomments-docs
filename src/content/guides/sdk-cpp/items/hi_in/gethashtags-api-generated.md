## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| page | double | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetHashTagsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getHashTags उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<double> page = 2.0;

api->getHashTags(tenantId, page).then([](pplx::task<std::shared_ptr<GetHashTagsResponse>> task) {
    try {
        auto resultPtr = task.get();
        auto response = std::make_shared<GetHashTagsResponse>(*resultPtr);
        // response का उपयोग करें
    } catch (const std::exception&) {
        // त्रुटि को संभालें
    }
});
[inline-code-end]