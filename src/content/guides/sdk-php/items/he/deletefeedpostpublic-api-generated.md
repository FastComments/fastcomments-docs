## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| postId | string | path | כן |  |
| broadcastId | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`DeleteFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteFeedPostPublic200Response.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמת deleteFeedPostPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש ב-client http מותאם אישית, העבר את ה-client שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$post_id = 'post_id_example'; // מחרוזת
$broadcast_id = 'broadcast_id_example'; // מחרוזת
$sso = 'sso_example'; // מחרוזת

try {
    $result = $apiInstance->deleteFeedPostPublic($tenant_id, $post_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---