## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | No |  |
| createHashTagBody | CreateHashTagBody | No |  |

## Response

Returns: [`AddHashTag_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddHashTag_200_response.h)

## Example

[inline-code-attrs-start title = 'addHashTag Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenantId = utility::string_t("my-tenant-123");
CreateHashTagBody body;
body.name = utility::string_t("release-notes");
body.color = utility::string_t("#FF5733");
boost::optional<CreateHashTagBody> createHashTagBody = body;
api->addHashTag(tenantId, createHashTagBody)
.then([](pplx::task<std::shared_ptr<AddHashTag_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto copied = std::make_shared<AddHashTag_200_response>(*resp);
        }
    } catch (const std::exception&) {}
});
[inline-code-end]
