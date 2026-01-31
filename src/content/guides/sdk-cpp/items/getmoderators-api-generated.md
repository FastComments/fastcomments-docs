## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | double | No |  |

## Response

Returns: [`GetModerators_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetModerators_200_response.h)

## Example

[inline-code-attrs-start title = 'getModerators Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 10.0;
api->getModerators(tenantId, skip)
.then([](pplx::task<std::shared_ptr<GetModerators_200_response>> t){
    try {
        auto resp = t.get();
        auto copy = std::make_shared<GetModerators_200_response>(*resp);
        std::cout << "Moderators retrieved: " << (resp ? "yes" : "no") << '\n';
    } catch (const std::exception &e) {
        std::cerr << "Error retrieving moderators: " << e.what() << '\n';
    }
}).wait();
[inline-code-end]
