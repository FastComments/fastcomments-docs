## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| id | string | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetV2PageReactUsersResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getV2PageReactUsers उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto urlId = utility::conversions::to_string_t("page-456");
auto id = utility::conversions::to_string_t("react-789");
boost::optional<utility::string_t> maybeFilter;
api->getV2PageReactUsers(tenantId, urlId, id).then([](pplx::task<std::shared_ptr<GetV2PageReactUsersResponse>> task){
    try{
        auto response = task.get();
        // प्रतिक्रिया का आवश्यकतानुसार उपयोग करें
    }catch(const std::exception&){
        // त्रुटि को संभालें
    }
});
[inline-code-end]