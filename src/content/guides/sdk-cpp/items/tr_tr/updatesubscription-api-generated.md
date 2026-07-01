## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | Evet |  |
| userId | string | Hayır |  |

## Yanıt

Döndürür: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateSubscriptionAPIResponse.h)

## Örnek

[inline-code-attrs-start title = 'updateSubscription Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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