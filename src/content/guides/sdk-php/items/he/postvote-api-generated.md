## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| direction | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## תגובה

מחזיר: [`VoteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteResponse.php)

## דוגמה

[inline-code-attrs-start title = 'postVote דוגמה'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // אם אתה רוצה להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // מחרוזת
$comment_id = 'comment_id_example'; // מחרוזת
$options = [
    'direction' => 'direction_example', // מחרוזת
    'broadcast_id' => 'broadcast_id_example', // מחרוזת
    'sso' => 'sso_example', // מחרוזת
];


try {
    $result = $apiInstance->postVote($tenant_id, $comment_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]