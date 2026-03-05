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
auto createPtr = std::make_shared<CreateAPISSOUserData>();
createPtr->email = utility::conversions::to_string_t("user@example.com");
createPtr->displayName = boost::optional<utility::string_t>(utility::conversions::to_string_t("Jane Doe"));
createPtr->externalId = boost::optional<utility::string_t>(utility::conversions::to_string_t("ext-456"));
createPtr->isAdmin = boost::optional<bool>(false);
api->addSSOUser(tenantId, *createPtr)
.then([](pplx::task<std::shared_ptr<AddSSOUserAPIResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            return resp;
        }
        return std::shared_ptr<AddSSOUserAPIResponse>();
    } catch (const std::exception&) {
        return std::shared_ptr<AddSSOUserAPIResponse>();
    }
});
[inline-code-end]
