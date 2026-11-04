מידע משתמשים בכמות גדולה עבור טננט. בהינתן userIds, מחזיר מידע תצוגה מ-User / SSOUser.
משמש את ווידג'ט התגובות להעשרת משתמשים שהופיעו זה עתה באמצעות אירוע נוכחות.
ללא הקשר עמוד: הפרטיות נאכפת באופן אחיד (פרופילים פרטיים מוסתרים).

## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| ids | string | query | כן | userIds מופרדים בפסיקים. |

## תגובה

מחזיר: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getUsersInfo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | userIds מופרדים בפסיקים.


try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]