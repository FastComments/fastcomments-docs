Omogućite ili onemogućite obaveštenja za stranicu. Kada su korisnici pretplaćeni na stranicu, obaveštenja se kreiraju
za nove korenske komentare, i takođe

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| url | string | Da |  |
| pageTitle | string | Da |  |
| subscribedOrUnsubscribed | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationPageSubscriptionStatusResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer updateUserNotificationPageSubscriptionStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> sso(utility::conversions::to_string_t("sso-token-abc123"));
api->updateUserNotificationPageSubscriptionStatus(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("article-456"),
    utility::conversions::to_string_t("https://www.example.com/posts/456"),
    utility::conversions::to_string_t("How to Test C++ SDK"),
    utility::conversions::to_string_t("subscribed"),
    sso
).then([](pplx::task<std::shared_ptr<UpdateUserNotificationPageSubscriptionStatusResponse>> t){
    try {
        auto resp = t.get();
        auto copy = std::make_shared<UpdateUserNotificationPageSubscriptionStatusResponse>(*resp);
        (void)copy;
    } catch (const std::exception&) { }
});
[inline-code-end]