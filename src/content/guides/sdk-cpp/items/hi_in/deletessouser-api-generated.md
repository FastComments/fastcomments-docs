## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| options | const DeleteSSOUserOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/DeleteSSOUserAPIResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'deleteSSOUser उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto userId = U("user-456");
DeleteSSOUserOptions options;
options.dryRun = boost::optional<bool>(true);
api->deleteSSOUser(tenantId, userId, options).then([](std::shared_ptr<DeleteSSOUserAPIResponse> resp) {
    if (resp) {
        (void)resp;
    }
});
[inline-code-end]