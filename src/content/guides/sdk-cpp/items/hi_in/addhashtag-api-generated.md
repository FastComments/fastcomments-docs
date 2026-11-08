## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createHashTagBody | CreateHashTagBody | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/CreateHashTagResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'addHashTag उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
CreateHashTagBody request;
request.tag = utility::conversions::to_string_t("feature-request");
request.description = utility::conversions::to_string_t("Requests for new features");
request.relatedUrl = boost::optional<utility::string_t>(utility::conversions::to_string_t("https://example.com/feature-request"));

api->addHashTag(utility::conversions::to_string_t("my-tenant-123"), request)
    .then([](std::shared_ptr<CreateHashTagResponse> resp) {
        (void)resp;
    });
[inline-code-end]