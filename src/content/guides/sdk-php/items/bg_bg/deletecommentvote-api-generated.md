## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| voteId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| broadcastId | string | query | Yes |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## Отговор

Връща: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteDeleteResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за deleteCommentVote'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате потребителски HTTP клиент, подайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, `GuzzleHttp\Client` ще се използва по подразбиране.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // тип: string
$comment_id = 'comment_id_example'; // тип: string
$vote_id = 'vote_id_example'; // тип: string
$url_id = 'url_id_example'; // тип: string
$broadcast_id = 'broadcast_id_example'; // тип: string
$edit_key = 'edit_key_example'; // тип: string
$sso = 'sso_example'; // тип: string

try {
    $result = $apiInstance->deleteCommentVote($tenant_id, $comment_id, $vote_id, $url_id, $broadcast_id, $edit_key, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteCommentVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]