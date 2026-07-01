Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial `{tenantId, urlId, commenterName}`  
index from afterName forward via $gt, no $skip cost.

## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | מזהה URL של הדף (נוקב בצד השרת). |
| afterName | string | query | No | מצביע: העבר nextAfterName מהתגובה הקודמת. |
| afterUserId | string | query | No | מפריד תחרות למצב מצביע: העבר nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקשרים באותו שם לא יוחזרו. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Example

[inline-code-attrs-start title = 'getOfflineUsers דוגמה'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם אתה רוצה להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמיישם `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש ברירת מחדל.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | מזהה URL של הדף (נוקב בצד השרת).
$options = [
    'after_name' => 'after_name_example', // string | מצביע: העבר nextAfterName מהתגובה הקודמת.
    'after_user_id' => 'after_user_id_example', // string | מפריד תחרות למצב מצביע: העבר nextAfterUserId מהתגובה הקודמת. נדרש כאשר afterName מוגדר כדי שמקשרים באותו שם לא יוחזרו.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]