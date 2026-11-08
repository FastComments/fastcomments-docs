## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetPendingWebhookEventCountOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetPendingWebhookEventCountResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getPendingWebhookEventCount उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
GetPendingWebhookEventCountOptions opts;
opts.filter = boost::optional<utility::string_t>(utility::conversions::to_string_t("active"));
api->getPendingWebhookEventCount(tenantId, opts)
    .then([](std::shared_ptr<GetPendingWebhookEventCountResponse> resp){
        auto count = resp->getCount();
    });
[inline-code-end]