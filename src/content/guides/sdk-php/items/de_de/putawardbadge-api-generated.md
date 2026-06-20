## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| badgeId | string | query | Ja |  |
| userId | string | query | Nein |  |
| commentId | string | query | Nein |  |
| broadcastId | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Rückgabe: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AwardUserBadgeResponse.php)

## Beispiel

[inline-code-attrs-start title = 'putAwardBadge Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional; standardmäßig wird `GuzzleHttp\Client` verwendet.
    new GuzzleHttp\Client()
);
$badge_id = 'badge_id_example'; // string
$user_id = 'user_id_example'; // string
$comment_id = 'comment_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->putAwardBadge($badge_id, $user_id, $comment_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->putAwardBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]