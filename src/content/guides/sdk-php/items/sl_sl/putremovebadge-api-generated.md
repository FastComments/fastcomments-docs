## Parametri

| Ime | Tip | Location | Obvezno | Opis |
|------|------|----------|----------|-------------|
| badgeId | string | query | Da |  |
| userId | string | query | Ne |  |
| commentId | string | query | Ne |  |
| broadcastId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrača: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/RemoveUserBadgeResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer putRemoveBadge'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Če želite uporabiti prilagojen HTTP odjemalec, posredujte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je izbirno, privzeto se uporabi `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$badge_id = 'badge_id_example'; // string
$user_id = 'user_id_example'; // string
$comment_id = 'comment_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->putRemoveBadge($badge_id, $user_id, $comment_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->putRemoveBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]