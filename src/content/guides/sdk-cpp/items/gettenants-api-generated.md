## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| meta | string | No |  |
| skip | double | No |  |

## Response

Returns: [`GetTenants_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenants_200_response.h)

## Example

[inline-code-attrs-start title = 'getTenants Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> meta = boost::optional<utility::string_t>(U("region=us-east"));
boost::optional<double> skip = boost::optional<double>(10.0);
api->getTenants(tenantId, meta, skip).then([](pplx::task<std::shared_ptr<GetTenants_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto localCopy = std::make_shared<GetTenants_200_response>(*resp);
        }
    } catch (const std::exception &ex) {
    }
});
[inline-code-end]
