## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | שאילתה | כן |  |
| sso | string | שאילתה | לא |  |

## תגובה

מחזיר: [`GetUserNotificationCount200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserNotificationCount200Response.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getUserNotificationCount'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$sso = 'sso_example'; // מחרוזת

try {
    $result = $apiInstance->getUserNotificationCount($tenant_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserNotificationCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]