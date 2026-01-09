## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | נתיב | כן |  |
| commentId | string | נתיב | כן |  |
| editKey | string | שאילתה | לא |  |
| sso | string | שאילתה | לא |  |

## תגובה

מחזיר: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentText200Response.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCommentText'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$comment_id = 'comment_id_example'; // מחרוזת
$edit_key = 'edit_key_example'; // מחרוזת
$sso = 'sso_example'; // מחרוזת

try {
    $result = $apiInstance->getCommentText($tenant_id, $comment_id, $edit_key, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentText: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]