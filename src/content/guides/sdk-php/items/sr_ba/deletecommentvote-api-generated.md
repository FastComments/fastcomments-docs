## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | путања | Да |  |
| commentId | string | путања | Да |  |
| voteId | string | путања | Да |  |
| urlId | string | упит | Да |  |
| broadcastId | string | упит | Да |  |
| editKey | string | упит | Не |  |
| sso | string | упит | Не |  |

## Одговор

Враћа: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteCommentVote200Response.php)

## Пример

[inline-code-attrs-start title = 'deleteCommentVote Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите користити прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, подразумевано ће бити `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // низ
$comment_id = 'comment_id_example'; // низ
$vote_id = 'vote_id_example'; // низ
$url_id = 'url_id_example'; // низ
$broadcast_id = 'broadcast_id_example'; // низ
$edit_key = 'edit_key_example'; // низ
$sso = 'sso_example'; // низ

try {
    $result = $apiInstance->deleteCommentVote($tenant_id, $comment_id, $vote_id, $url_id, $broadcast_id, $edit_key, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteCommentVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]