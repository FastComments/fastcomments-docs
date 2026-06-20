## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | כן |  |
| voteId | string | path | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteDeleteResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteModerationVote'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$vote_id = 'vote_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->deleteModerationVote($comment_id, $vote_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->deleteModerationVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]