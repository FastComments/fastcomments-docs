## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | No |  |
| commentId | string | query | No |  |
| sso | string | query | No |  |

## תגובה

מחזיר: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserManualBadgesResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getManualBadgesForUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$badges_user_id = 'badges_user_id_example'; // מחרוזת
$comment_id = 'comment_id_example'; // מחרוזת
$sso = 'sso_example'; // מחרוזת

try {
    $result = $apiInstance->getManualBadgesForUser($badges_user_id, $comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getManualBadgesForUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]