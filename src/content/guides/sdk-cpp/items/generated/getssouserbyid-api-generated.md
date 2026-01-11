## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetSSOUserByIdAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSSOUserByIdAPIResponse.h)

## Example

[inline-code-attrs-start title = 'getSSOUserById Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t userId = utility::conversions::to_string_t("user@example.com");
boost::optional<utility::string_t> tenantOpt = tenantId;
api->getSSOUserById(tenantOpt.value(), userId)
    .then([](std::shared_ptr<GetSSOUserByIdAPIResponse> resp){
        if(!resp) return;
        auto responseCopy = std::make_shared<GetSSOUserByIdAPIResponse>(*resp);
    });
[inline-code-end]
