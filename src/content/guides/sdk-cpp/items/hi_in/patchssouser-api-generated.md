## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| updateAPISSOUserData | UpdateAPISSOUserData | हाँ |  |
| updateComments | bool | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PatchSSOUserAPIResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'patchSSOUser उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user-456");
UpdateAPISSOUserData updateData;
updateData.email = U("jane.doe@example.com");
updateData.displayName = U("Jane Doe");
boost::optional<bool> updateComments = true;

api->patchSSOUser(tenantId, userId, updateData, updateComments)
    .then([](pplx::task<std::shared_ptr<PatchSSOUserAPIResponse>> t){
        try{
            auto resp = t.get();
            (void)resp;
        }catch(const std::exception&){
        }
    });
[inline-code-end]