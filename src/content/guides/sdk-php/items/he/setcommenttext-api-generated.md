## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| commentId | string | path | כן |  |
| broadcastId | string | query | כן |  |
| editKey | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PublicAPISetCommentTextResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמת setCommentText'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש ב-client HTTP מותאם, העבר את ה-client שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$comment_id = 'comment_id_example'; // מחרוזת
$broadcast_id = 'broadcast_id_example'; // מחרוזת
$comment_text_update_request = new \FastComments\Client\Model\CommentTextUpdateRequest(); // \FastComments\Client\Model\CommentTextUpdateRequest
$edit_key = 'edit_key_example'; // מחרוזת
$sso = 'sso_example'; // מחרוזת

try {
    $result = $apiInstance->setCommentText($tenant_id, $comment_id, $broadcast_id, $comment_text_update_request, $edit_key, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->setCommentText: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]