## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| commentId | string | path | כן |  |
| urlId | string | query | כן |  |
| broadcastId | string | query | כן |  |
| sessionId | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`VoteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteResponse.php)

## דוגמה

[inline-code-attrs-start title = 'voteComment דוגמה'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם אתה רוצה להשתמש ב‑client HTTP מותאם, העבר את ה‑client שלך שמממש `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` יופעל כברירת מחדל.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // מחרוזת
$comment_id = 'comment_id_example'; // מחרוזת
$url_id = 'url_id_example'; // מחרוזת
$broadcast_id = 'broadcast_id_example'; // מחרוזת
$vote_body_params = new \FastComments\Client\Model\VoteBodyParams(); // \FastComments\Client\Model\VoteBodyParams
$options = [
    'session_id' => 'session_id_example', // מחרוזת
    'sso' => 'sso_example', // מחרוזת
];


try {
    $result = $apiInstance->voteComment($tenant_id, $comment_id, $url_id, $broadcast_id, $vote_body_params, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception כאשר קוראים ל‑PublicApi->voteComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---