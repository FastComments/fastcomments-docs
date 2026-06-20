מצביעים קודמים בעמוד שאינם מקוונים כרגע. ממוין לפי displayName.
השתמש בזה לאחר שנגמרו התוצאות של /users/online כדי להציג מדור 'חברים'.
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}
index from afterName forward via $gt, no $skip cost.

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | מזהה URL של הדף (מנוקה בצד השרת). |
| afterName | string | query | No | סמן: העבר את nextAfterName מהתגובה הקודמת. |
| afterUserId | string | query | No | שובר תיקו לסמן: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של שמות שווים לא יגרמו לאובדן רשומות. |

## תגובה

מחזיר: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי; `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | מזהה URL של הדף (מנוקה בצד השרת).
$after_name = 'after_name_example'; // string | סמן: העבר את nextAfterName מהתגובה הקודמת.
$after_user_id = 'after_user_id_example'; // string | שובר תיקו לסמן: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של שמות שווים לא יגרמו לאובדן רשומות.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]