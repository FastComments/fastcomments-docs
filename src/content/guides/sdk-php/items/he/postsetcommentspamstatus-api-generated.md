## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | נתיב | כן |  |
| spam | boolean | שאילתה | לא |  |
| permNotSpam | boolean | שאילתה | לא |  |
| sso | string | שאילתה | לא |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמת postSetCommentSpamStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // אם ברצונך להשתמש ב-client HTTP מותאם אישית, העבר את הלקוח שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // מחרוזת
$spam = True; // בוליאני
$perm_not_spam = True; // בוליאני
$sso = 'sso_example'; // מחרוזת

try {
    $result = $apiInstance->postSetCommentSpamStatus($comment_id, $spam, $perm_not_spam, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postSetCommentSpamStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---