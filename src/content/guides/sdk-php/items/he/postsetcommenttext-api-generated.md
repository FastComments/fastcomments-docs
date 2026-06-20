## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | נתיב | כן |  |
| sso | string | שאילתה | לא |  |

## תגובה

מחזיר: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SetCommentTextResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-postSetCommentText'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // מחרוזת
$set_comment_text_params = new \FastComments\Client\Model\SetCommentTextParams(); // \FastComments\Client\Model\SetCommentTextParams
$sso = 'sso_example'; // מחרוזת

try {
    $result = $apiInstance->postSetCommentText($comment_id, $set_comment_text_params, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postSetCommentText: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---