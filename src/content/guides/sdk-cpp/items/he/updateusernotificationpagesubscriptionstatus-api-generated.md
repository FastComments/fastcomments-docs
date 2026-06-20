---
הפעל או השבת התראות עבור דף. כאשר משתמשים מנויים לדף, נוצרים התראות עבור תגובות שורש חדשות, וגם

## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| url | string | כן |  |
| pageTitle | string | כן |  |
| subscribedOrUnsubscribed | string | כן |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationPageSubscriptionStatusResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל־updateUserNotificationPageSubscriptionStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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

---