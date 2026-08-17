Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
צופים מקוונים כרגע של דף: אנשים שהחיבור WebSocket שלהם מנוי לדף ברגע זה.

Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).  
מחזיר anonCount + totalCount (מנויים ברמת החדר, כולל צופים אנונימיים שאינם נספרים).

## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | מזהה כתובת URL של הדף (נוקה בצד השרת). |
| afterName | string | query | No | סמן: העבר nextAfterName מהתגובה הקודמת. |
| afterUserId | string | query | No | קשר שבירת תחרות של סמן: העבר nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של שוויון בשם לא יפלו מהתוצאות. |

## Response

מחזיר: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Example

[inline-code-attrs-start title = 'getOnlineUsers דוגמה'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש ברירת מחדל.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | מזהה כתובת URL של הדף (נוקה בצד השרת).
$options = [
    'after_name' => 'after_name_example', // string | סמן: העבר nextAfterName מהתגובה הקודמת.
    'after_user_id' => 'after_user_id_example', // string | קשר שבירת תחרות של סמן: העבר nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקרים של שוויון בשם לא יפלו מהתוצאות.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---