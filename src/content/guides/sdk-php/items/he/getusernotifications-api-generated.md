## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| pageSize | integer | query | לא |  |
| afterId | string | query | לא |  |
| includeContext | boolean | query | לא |  |
| afterCreatedAt | integer | query | לא |  |
| unreadOnly | boolean | query | לא |  |
| dmOnly | boolean | query | לא |  |
| noDm | boolean | query | לא |  |
| includeTranslations | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserNotifications200Response.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getUserNotifications'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש ב-client HTTP מותאם אישית, העבר את ה-client שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$page_size = 56; // int
$after_id = 'after_id_example'; // string
$include_context = True; // bool
$after_created_at = 56; // int
$unread_only = True; // bool
$dm_only = True; // bool
$no_dm = True; // bool
$include_translations = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getUserNotifications($tenant_id, $page_size, $after_id, $include_context, $after_created_at, $unread_only, $dm_only, $no_dm, $include_translations, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]