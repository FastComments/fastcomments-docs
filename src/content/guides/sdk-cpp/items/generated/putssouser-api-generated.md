## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Yes |  |
| updateComments | bool | No |  |

## Response

Returns: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PutSSOUserAPIResponse.h)

## Example

[inline-code-attrs-start title = 'putSSOUser Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
UpdateAPISSOUserData ssoData;
ssoData.email = utility::string_t("user@example.com");
ssoData.name = utility::string_t("Jane Doe");
boost::optional<bool> updateComments = true;
api->putSSOUser(utility::string_t("my-tenant-123"), utility::string_t("user@example.com"), ssoData, updateComments)
.then([](pplx::task<std::shared_ptr<PutSSOUserAPIResponse>> task){
    try {
        auto resp = task.get();
        auto result = resp ? std::make_shared<PutSSOUserAPIResponse>(*resp) : std::shared_ptr<PutSSOUserAPIResponse>();
        (void)result;
    } catch(const std::exception &e) {
        (void)e;
    }
});
[inline-code-end]
