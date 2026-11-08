## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | हाँ |  |
| userId | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/UpdateSubscriptionAPIResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'updateSubscription उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
UpdateAPIUserSubscriptionData subscriptionData;
subscriptionData.plan = utility::conversions::to_string_t("premium");
subscriptionData.active = true;

api->updateSubscription(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("sub-987654"),
    subscriptionData,
    boost::optional<utility::string_t>(utility::conversions::to_string_t("admin-user-456"))
).then([](std::shared_ptr<UpdateSubscriptionAPIResponse> response){
    bool ok = response && response->isSuccess;
    (void)ok;
});
[inline-code-end]