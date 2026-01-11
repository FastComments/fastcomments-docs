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
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t userId = utility::conversions::to_string_t("user@example.com");
boost::optional<utility::string_t> tenantOpt(tenantId);
boost::optional<utility::string_t> userOpt;
api->getUserBadgeProgressByUserId(tenantOpt.value_or(tenantId), userOpt.value_or(userId))
  .then([](pplx::task<std::shared_ptr<GetUserBadgeProgressById_200_response>> t){
    try {
      auto resp = t.get();
      if (!resp) resp = std::make_shared<GetUserBadgeProgressById_200_response>();
    } catch (const std::exception&) {}
  });
[inline-code-end]
