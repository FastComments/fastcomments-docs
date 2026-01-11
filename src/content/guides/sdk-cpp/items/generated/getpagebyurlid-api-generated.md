## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Response

Returns: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPageByURLIdAPIResponse.h)

## Example

[inline-code-attrs-start title = 'getPageByURLId Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> optUrlId = utility::conversions::to_string_t("page-9f8b7c6e");
auto reqTask = api->getPageByURLId(tenantId, *optUrlId)
    .then([](pplx::task<std::shared_ptr<GetPageByURLIdAPIResponse>> task)
    {
        try
        {
            auto resp = task.get();
            if (!resp) resp = std::make_shared<GetPageByURLIdAPIResponse>();
        }
        catch (const std::exception&)
        {
            auto resp = std::make_shared<GetPageByURLIdAPIResponse>();
        }
    });
[inline-code-end]
