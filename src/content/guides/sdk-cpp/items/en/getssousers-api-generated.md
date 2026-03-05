## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | int32_t | No |  |

## Response

Returns: [`GetSSOUsers_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSSOUsers_200_response.h)

## Example

[inline-code-attrs-start title = 'getSSOUsers Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<int32_t> skip = boost::optional<int32_t>(20);
api->getSSOUsers(tenantId, skip)
    .then([=](pplx::task<std::shared_ptr<GetSSOUsers_200_response>> task){
        try {
            auto resp = task.get();
            if(!resp) resp = std::make_shared<GetSSOUsers_200_response>();
            std::cout << "Fetched SSO users successfully\n";
        } catch(const std::exception& e) {
            std::cerr << "Error fetching SSO users: " << e.what() << '\n';
        }
    });
[inline-code-end]
