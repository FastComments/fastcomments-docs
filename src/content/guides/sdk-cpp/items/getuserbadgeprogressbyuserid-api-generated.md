## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |

## Response

Returns: [`GetUserBadgeProgressById_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserBadgeProgressById_200_response.h)

## Example

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> locale = boost::optional<utility::string_t>(U("en-US"));
auto samplePtr = std::make_shared<GetUserBadgeProgressById_200_response>();
pplx::task<void> t = api->getUserBadgeProgressByUserId(tenantId, userId)
    .then([](pplx::task<std::shared_ptr<GetUserBadgeProgressById_200_response>> previousTask) {
        try {
            auto resp = previousTask.get();
            if (resp) {
                std::wcout << U("Badge progress received for user") << std::endl;
            } else {
                std::wcout << U("No badge progress available") << std::endl;
            }
        } catch (const std::exception &e) {
            std::cerr << "Error calling getUserBadgeProgressByUserId: " << e.what() << std::endl;
        }
    });
[inline-code-end]
