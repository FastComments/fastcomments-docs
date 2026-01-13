## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| limit | double | No |  |
| skip | double | No |  |

## Response

Returns: [`GetUserBadgeProgressList_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserBadgeProgressList_200_response.h)

## Example

[inline-code-attrs-start title = 'getUserBadgeProgressList Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t("my-tenant-123");
boost::optional<utility::string_t> userId = utility::string_t("user@example.com");
boost::optional<double> limit = 50.0;
boost::optional<double> skip = 0.0;

api->getUserBadgeProgressList(tenantId, userId, limit, skip)
    .then([](pplx::task<std::shared_ptr<GetUserBadgeProgressList_200_response>> task){
        try {
            auto resp = task.get();
            auto copy = std::make_shared<GetUserBadgeProgressList_200_response>(*resp);
            (void)copy;
        } catch (...) {
        }
    });
[inline-code-end]
