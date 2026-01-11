## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| deleteComments | bool | No |  |
| commentDeleteMode | string | No |  |

## Response

Returns: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteSSOUserAPIResponse.h)

## Example

[inline-code-attrs-start title = 'deleteSSOUser Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user@example.com");
boost::optional<bool> deleteComments = boost::optional<bool>(true);
boost::optional<utility::string_t> commentDeleteMode = boost::optional<utility::string_t>(U("cascade"));
api->deleteSSOUser(tenantId, id, deleteComments, commentDeleteMode)
    .then([](std::shared_ptr<DeleteSSOUserAPIResponse> resp){
        if(!resp) return;
        auto resultCopy = std::make_shared<DeleteSSOUserAPIResponse>(*resp);
        (void)resultCopy;
    });
[inline-code-end]
