הפעל או כבה התראות עבור דף. כאשר משתמשים מנויים לדף, נוצרות התראות
לתגובות שורש חדשות, וגם

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlId | string | query | כן |  |
| url | string | query | כן |  |
| pageTitle | string | query | כן |  |
| subscribedOrUnsubscribed | string | path | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserNotificationPageSubscriptionStatusResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateUserNotificationPageSubscriptionStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$url_id = 'url_id_example'; // מחרוזת
$url = 'url_example'; // מחרוזת
$page_title = 'page_title_example'; // מחרוזת
$subscribed_or_unsubscribed = 'subscribed_or_unsubscribed_example'; // מחרוזת
$sso = 'sso_example'; // מחרוזת

try {
    $result = $apiInstance->updateUserNotificationPageSubscriptionStatus($tenant_id, $url_id, $url, $page_title, $subscribed_or_unsubscribed, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateUserNotificationPageSubscriptionStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]