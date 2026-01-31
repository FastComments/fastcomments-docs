## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | No |  |
| externalId | string | No |  |
| eventType | string | No |  |
| type | string | No |  |
| domain | string | No |  |
| attemptCountGT | double | No |  |
| skip | double | No |  |

## Response

Returns: [`GetPendingWebhookEvents_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPendingWebhookEvents_200_response.h)

## Example

[inline-code-attrs-start title = 'getPendingWebhookEvents Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> commentId(U("comment-987"));
boost::optional<utility::string_t> externalId(U("ext-555"));
boost::optional<utility::string_t> eventType(U("comment.created"));
boost::optional<utility::string_t> typeOpt(U("webhook"));
boost::optional<utility::string_t> domain(U("example.com"));
boost::optional<double> attemptCountGT(2.0);
boost::optional<double> skip(0.0);

api->getPendingWebhookEvents(tenantId, commentId, externalId, eventType, typeOpt, domain, attemptCountGT, skip)
.then([](pplx::task<std::shared_ptr<GetPendingWebhookEvents_200_response>> t)
{
    try
    {
        auto resp = t.get();
        auto safeResp = resp ? resp : std::make_shared<GetPendingWebhookEvents_200_response>();
        (void)safeResp;
    }
    catch (const std::exception&)
    {
        auto empty = std::make_shared<GetPendingWebhookEvents_200_response>();
        (void)empty;
    }
});
[inline-code-end]
