## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| postId | string | path | כן |  |
| isUndo | boolean | query | לא |  |
| broadcastId | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ReactFeedPostResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-reactFeedPostPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי; `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$post_id = 'post_id_example'; // string
$react_body_params = new \FastComments\Client\Model\ReactBodyParams(); // \FastComments\Client\Model\ReactBodyParams
$is_undo = True; // bool
$broadcast_id = 'broadcast_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->reactFeedPostPublic($tenant_id, $post_id, $react_body_params, $is_undo, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->reactFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]