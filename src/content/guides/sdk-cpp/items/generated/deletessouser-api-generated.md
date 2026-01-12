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
boost::optional<bool> deleteComments(true);
boost::optional<utility::string_t> commentDeleteMode(U("soft_delete"));

api->deleteSSOUser(tenantId, id, deleteComments, commentDeleteMode)
    .then([](pplx::task<std::shared_ptr<DeleteSSOUserAPIResponse>> t) {
        try {
            auto resp = t.get();
            auto fallback = std::make_shared<DeleteSSOUserAPIResponse>();
            if (resp) { (void)resp; } else { (void)fallback; }
        } catch (const std::exception&) {
        }
    });
[inline-code-end]
