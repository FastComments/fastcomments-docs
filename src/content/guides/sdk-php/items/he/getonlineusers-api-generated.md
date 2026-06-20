צופים שמחוברים כרגע לעמוד: אנשים שסשן ה-websocket שלהם מנוי לעמוד כרגע.
מחזיר anonCount + totalCount (מנויים בחדר כולו, כולל צופים אנונימיים שאותם איננו מונים).

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | מזהה URL של העמוד (נקי בצד השרת). |
| afterName | string | query | No | סמן: העבר את nextAfterName מהתגובה הקודמת. |
| afterUserId | string | query | No | בורר תיקו של הסמן: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שלא ייפלו רשומות במקרה של שמות זהים. |

## תגובה

מחזיר: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | מזהה URL של העמוד (נקי בצד השרת).
$after_name = 'after_name_example'; // string | סמן: העבר את nextAfterName מהתגובה הקודמת.
$after_user_id = 'after_user_id_example'; // string | בורר תיקו של הסמן: העבר את nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שלא ייפלו רשומות במקרה של שמות זהים.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]