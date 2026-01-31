## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentIds | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`CheckedCommentsForBlocked_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CheckedCommentsForBlocked_200_response.h)

## Example

[inline-code-attrs-start title = 'checkedCommentsForBlocked Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto commentIds = utility::string_t(U("cmt-456,cmt-789"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->checkedCommentsForBlocked(tenantId, commentIds, sso)
    .then([](std::shared_ptr<CheckedCommentsForBlocked_200_response> resp) {
        if (resp) {
            auto localCopy = std::make_shared<CheckedCommentsForBlocked_200_response>(*resp);
        }
    })
    .wait();
[inline-code-end]
