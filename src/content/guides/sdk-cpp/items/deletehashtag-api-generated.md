## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | Yes |  |
| tenantId | string | No |  |
| deleteHashTagRequest | DeleteHashTag_request | No |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteHashTag Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tag = U("off-topic");
boost::optional<utility::string_t> tenantId = boost::optional<utility::string_t>(U("my-tenant-123"));
boost::optional<DeleteHashTag_request> deleteReq = boost::optional<DeleteHashTag_request>(DeleteHashTag_request());
api->deleteHashTag(tag, tenantId, deleteReq).then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto copy = std::make_shared<FlagCommentPublic_200_response>(*resp);
        }
    } catch (...) {}
});
[inline-code-end]
