## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| sure | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'deleteTenant उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto userId = utility::conversions::to_string_t("user@example.com");
boost::optional<utility::string_t> sure = utility::conversions::to_string_t("true");
api->deleteTenant(tenantId, userId, sure)
   .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t){
       try{
           auto resp = t.get();
           auto result = std::make_shared<APIEmptyResponse>(*resp);
       }catch(const std::exception&){})
[inline-code-end]