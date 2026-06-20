## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | נתיב | כן |  |
| commentId | string | נתיב | כן |  |
| urlId | string | שאילתה | כן |  |
| broadcastId | string | שאילתה | כן |  |
| sessionId | string | שאילתה | לא |  |
| sso | string | שאילתה | לא |  |

## תגובה

מחזיר: [`VoteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-voteComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$url_id = 'url_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$vote_body_params = new \FastComments\Client\Model\VoteBodyParams(); // \FastComments\Client\Model\VoteBodyParams
$session_id = 'session_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->voteComment($tenant_id, $comment_id, $url_id, $broadcast_id, $vote_body_params, $session_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->voteComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]