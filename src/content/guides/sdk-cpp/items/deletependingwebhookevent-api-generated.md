## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'deletePendingWebhookEvent Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t eventId = utility::conversions::to_string_t("wh-event-789");
boost::optional<utility::string_t> optTenant(tenantId);
api->deletePendingWebhookEvent(optTenant.value(), eventId)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t){
    try {
        auto result = t.get();
        auto resp = result ? result : std::make_shared<FlagCommentPublic_200_response>();
        (void)resp;
    } catch (...) {}
});
[inline-code-end]
