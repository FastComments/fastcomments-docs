## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| commentId | string | path | כן |  |
| includeEmail | boolean | query | לא |  |
| includeIP | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPICommentResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getModerationComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, ישתמש ב-`GuzzleHttp\Client` כברירת מחדל.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // מחרוזת
$include_email = True; // בוליאני
$include_ip = True; // בוליאני
$sso = 'sso_example'; // מחרוזת

try {
    $result = $apiInstance->getModerationComment($comment_id, $include_email, $include_ip, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getModerationComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---