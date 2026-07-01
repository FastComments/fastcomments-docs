## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const PostApiExportOptions& | Yes |  |

## Response

Returns: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationExportResponse.h)

## Example

[inline-code-attrs-start title = 'postApiExport Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
PostApiExportOptions options;
options.format = utility::string_t(U("json"));
options.email = utility::string_t(U("reports@example.com"));
options.startDate = boost::optional<utility::datetime>(utility::datetime::from_string(U("2023-01-01T00:00:00Z")));
options.endDate = boost::optional<utility::datetime>(utility::datetime::from_string(U("2023-01-31T23:59:59Z")));

api->postApiExport(tenantId, options)
    .then([](std::shared_ptr<ModerationExportResponse> response) {
        if (response) {
            // handle successful export response
        }
    })
    .wait();
[inline-code-end]
