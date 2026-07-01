## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | Ja |  |
| userId | string | Nee |  |

## Respons

Retourneert: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateSubscriptionAPIResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'updateSubscription Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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