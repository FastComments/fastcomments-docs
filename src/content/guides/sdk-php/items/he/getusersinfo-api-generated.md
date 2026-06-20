---
מידע מרוכז על משתמשים עבור tenant. בהינתן userIds, מחזיר מידע להצגה מ-User / SSOUser.
משמש ב-widget של תגובות כדי להעשיר משתמשים שהופיעו זה עתה דרך אירוע נוכחות.
אין הקשר של עמוד: פרטיות נאכפת באופן אחיד (פרופילים פרטיים מטושטשים).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| ids | string | query | כן | userIds מופרדים בפסיקים. |

## Response

Returns: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Example

[inline-code-attrs-start title = 'דוגמת getUsersInfo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
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

---