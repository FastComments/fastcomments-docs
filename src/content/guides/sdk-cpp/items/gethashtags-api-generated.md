## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| page | double | No |  |

## Response

Returns: [`GetHashTags_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetHashTags_200_response.h)

## Example

[inline-code-attrs-start title = 'getHashTags Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> page = 2.0;
api->getHashTags(tenantId, page).then([](pplx::task<std::shared_ptr<GetHashTags_200_response>> task) {
    try {
        auto resp = task.get();
        auto result = resp ? resp : std::make_shared<GetHashTags_200_response>();
        std::cout << "Received response: " << (resp != nullptr) << std::endl;
    } catch (const std::exception &e) {
        std::cerr << "getHashTags failed: " << e.what() << std::endl;
    }
});
[inline-code-end]
