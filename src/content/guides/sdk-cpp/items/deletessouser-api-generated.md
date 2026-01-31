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
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t id = utility::conversions::to_string_t("user@example.com");
boost::optional<bool> deleteComments = true;
boost::optional<utility::string_t> commentDeleteMode = utility::conversions::to_string_t("hard");
api->deleteSSOUser(tenantId, id, deleteComments, commentDeleteMode)
    .then([](pplx::task<std::shared_ptr<DeleteSSOUserAPIResponse>> t)
    {
        try
        {
            auto resp = t.get();
            if (!resp) resp = std::make_shared<DeleteSSOUserAPIResponse>();
            return resp;
        }
        catch (...)
        {
            return std::make_shared<DeleteSSOUserAPIResponse>();
        }
    });
[inline-code-end]
