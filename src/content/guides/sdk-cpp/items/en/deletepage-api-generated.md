## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`DeletePageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeletePageAPIResponse.h)

## Example

[inline-code-attrs-start title = 'deletePage Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t pageId = utility::conversions::to_string_t("page-456");
boost::optional<utility::string_t> auditNote = utility::conversions::to_string_t("admin@company.com requested deletion");
api->deletePage(tenantId, pageId)
    .then([auditNote](pplx::task<std::shared_ptr<DeletePageAPIResponse>> t) {
        try {
            auto resp = t.get();
            auto result = std::make_shared<DeletePageAPIResponse>(*resp);
            if (auditNote) {
                (void)auditNote;
            }
        } catch (const std::exception&) {
        }
    });
[inline-code-end]
