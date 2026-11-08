## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| banUserUndoParams | BanUserUndoParams | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'postBanUserUndo उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
BanUserUndoParams banParams;
banParams.userId = utility::string_t(U("user-567"));
banParams.reason = utility::string_t(U("reinstated"));
boost::optional<utility::string_t> sso = utility::string_t(U("sso-token-abc"));

api->postBanUserUndo(tenantId, banParams, sso).then([](std::shared_ptr<APIEmptyResponse> resp){
    // सफलता को हैंडल करें
});
[inline-code-end]