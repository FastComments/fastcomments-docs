## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetManualBadgesForUserOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetUserManualBadgesResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getManualBadgesForUser उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetManualBadgesForUserOptions options;
options.userEmail = boost::optional<utility::string_t>(U("user@example.com"));
options.includeInactive = boost::optional<bool>(true);
api->getManualBadgesForUser(tenantId, options).then([](pplx::task<std::shared_ptr<GetUserManualBadgesResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]