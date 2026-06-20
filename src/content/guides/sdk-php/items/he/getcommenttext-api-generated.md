## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## תגובה

מחזיר: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PublicAPIGetCommentTextResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCommentText'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שמממש את `GuzzleHttp\ClientInterface`.
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

---