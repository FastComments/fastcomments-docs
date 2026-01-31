## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPagesAPIResponse.h)

## Example

[inline-code-attrs-start title = 'getPages Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<int> limit = 50;
boost::optional<utility::string_t> cursor = boost::none;
api->getPages(tenantId).then([=](std::shared_ptr<GetPagesAPIResponse> resp){
    auto responseCopy = std::make_shared<GetPagesAPIResponse>(*resp);
    return responseCopy;
}).then([](std::shared_ptr<GetPagesAPIResponse> finalResp){
    if (finalResp) {
        std::cout << "Pages fetched for tenant\n";
    }
});
[inline-code-end]
