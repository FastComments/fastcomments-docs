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
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentIds = U("cmt-456,cmt-789");
boost::optional<utility::string_t> sso{ U("user@example.com") };
api->checkedCommentsForBlocked(tenantId, commentIds, sso)
.then([](std::shared_ptr<CheckedCommentsForBlocked_200_response> resp){
    auto copied = std::make_shared<CheckedCommentsForBlocked_200_response>(*resp);
    return copied;
})
.then([](std::shared_ptr<CheckedCommentsForBlocked_200_response> finalResult){
    (void)finalResult;
})
.wait();
[inline-code-end]
