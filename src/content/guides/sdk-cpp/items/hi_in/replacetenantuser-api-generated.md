## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| replaceTenantUserBody | ReplaceTenantUserBody | हाँ |  |
| updateComments | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'replaceTenantUser उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto userId = utility::string_t(U("john.doe@example.com"));
ReplaceTenantUserBody replaceBody;
replaceBody.email = utility::string_t(U("john.doe@example.com"));
replaceBody.role = utility::string_t(U("admin"));
boost::optional<utility::string_t> updateComments = utility::string_t(U("Promoted to admin"));
api->replaceTenantUser(tenantId, userId, replaceBody, updateComments)
    .then([](std::shared_ptr<APIEmptyResponse> resp){ });
[inline-code-end]