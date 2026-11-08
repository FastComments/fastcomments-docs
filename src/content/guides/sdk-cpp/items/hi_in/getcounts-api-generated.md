## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetBannedUsersCountResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getCounts उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->getCounts(
    utility::conversions::to_string_t("my-tenant-123"),
    boost::optional<utility::string_t>(utility::conversions::to_string_t("john.doe@example.com"))
).then([](pplx::task<std::shared_ptr<GetBannedUsersCountResponse>> t){
    try{
        auto response = t.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]