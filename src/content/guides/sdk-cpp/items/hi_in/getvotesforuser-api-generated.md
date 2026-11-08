## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| options | const GetVotesForUserOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetVotesForUserResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getVotesForUser उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto urlId = utility::string_t(U("post-456"));
GetVotesForUserOptions options;
options.page = boost::optional<int>(2);
options.pageSize = boost::optional<int>(50);
api->getVotesForUser(tenantId, urlId, options).then([](std::shared_ptr<GetVotesForUserResponse> response) {
    if (response) {
        // प्रतिक्रिया संभालें, जैसे, वोट्स पर पुनरावृति करें
    }
});
[inline-code-end]