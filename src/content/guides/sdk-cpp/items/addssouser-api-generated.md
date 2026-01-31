## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPISSOUserData | CreateAPISSOUserData | Yes |  |

## Response

Returns: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddSSOUserAPIResponse.h)

## Example

[inline-code-attrs-start title = 'addSSOUser Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
CreateAPISSOUserData createData;
createData.email = utility::conversions::to_string_t("user@example.com");
createData.displayName = utility::conversions::to_string_t("Jane Doe");
createData.externalId = utility::conversions::to_string_t("okta-12345");
createData.roles = std::vector<utility::string_t>{ utility::conversions::to_string_t("member") };
createData.metadata = boost::optional<utility::string_t>(utility::conversions::to_string_t("preferred_language=en"));

auto task = api->addSSOUser(tenantId, createData)
    .then([](std::shared_ptr<AddSSOUserAPIResponse> resp){
        if(resp){
            auto copy = std::make_shared<AddSSOUserAPIResponse>(*resp);
        }
    });

task.wait();
[inline-code-end]
