## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlIdWS | string | query | כן |  |
| userIds | string | query | כן |  |

## תגובה

מחזיר: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserPresenceStatuses200Response.php)

## דוגמה

[inline-code-attrs-start title = 'getUserPresenceStatuses דוגמה'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כבררת מחדל.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$url_id_ws = 'url_id_ws_example'; // מחרוזת
$user_ids = 'user_ids_example'; // מחרוזת

try {
    $result = $apiInstance->getUserPresenceStatuses($tenant_id, $url_id_ws, $user_ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserPresenceStatuses: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]