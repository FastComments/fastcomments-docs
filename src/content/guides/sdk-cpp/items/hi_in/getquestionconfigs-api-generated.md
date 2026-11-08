## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| skip | double | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetQuestionConfigsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getQuestionConfigs उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<double> skip = 10.0;
api->getQuestionConfigs(tenantId, skip).then([](std::shared_ptr<GetQuestionConfigsResponse> resp){
    auto config = std::make_shared<GetQuestionConfigsResponse>(*resp);
});
[inline-code-end]