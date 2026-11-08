## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetUserInternalProfileOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetUserInternalProfileResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getUserInternalProfile उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetUserInternalProfileOptions options;
options.email = boost::optional<utility::string_t>(U("user@example.com"));
options.includeDetails = boost::optional<bool>(true);

api->getUserInternalProfile(tenantId, options)
    .then([](std::shared_ptr<GetUserInternalProfileResponse> response) {
        if (response) {
            auto name = response->displayName;
            auto id = response->userId;
        }
    });
[inline-code-end]