Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
מחזיר anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).  
מחזיר anonCount + totalCount (מנויים בכל החדר, כולל צופים אנונימיים שאנו לא סופרים).

## Parameters  
## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Page URL identifier (cleaned server-side). |
| urlId | string | query | Yes | מזהה כתובת URL של הדף (ניקוי בצד השרת). |
| afterName | string | query | No | Cursor: pass nextAfterName from the previous response. |
| afterName | string | query | No | סמן: העבר nextAfterName מהתגובה הקודמת. |
| afterUserId | string | query | No | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |
| afterUserId | string | query | No | שובר קושי של סמן: העבר nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקשרים על שם לא יפלו מהתוצאות. |

## Response  
## תגובה

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)  
מחזיר: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Example  
## דוגמה

[inline-code-attrs-start title = 'getOnlineUsers דוגמה'; type = 'php'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
<?php  
require_once(__DIR__ . '/vendor/autoload.php');  



$apiInstance = new FastComments\Client\Api\PublicApi(  
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמיישם `GuzzleHttp\ClientInterface`.  
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.  
    new GuzzleHttp\Client()  
);  

$tenant_id = 'tenant_id_example'; // string  
$url_id = 'url_id_example'; // string | Page URL identifier (cleaned server-side).  
$url_id = 'url_id_example'; // string | מזהה כתובת URL של הדף (ניקוי בצד השרת).  
$options = [  
    'after_name' => 'after_name_example', // string | Cursor: pass nextAfterName from the previous response.  
    'after_name' => 'after_name_example', // string | סמן: העבר nextAfterName מהתגובה הקודמת.  
    'after_user_id' => 'after_user_id_example', // string | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries.  
    'after_user_id' => 'after_user_id_example', // string | שובר קושי של סמן: העבר nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקשרים על שם לא יפלו מהתוצאות.  
];  


try {  
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);  
    print_r($result);  
} catch (Exception $e) {  
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;  
}  
[inline-code-end]