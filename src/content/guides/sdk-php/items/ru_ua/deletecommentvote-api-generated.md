## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| commentId | string | path | Да |  |
| voteId | string | path | Да |  |
| urlId | string | query | Да |  |
| broadcastId | string | query | Да |  |
| editKey | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteCommentVote200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример deleteCommentVote'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать кастомный HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // строка
$comment_id = 'comment_id_example'; // строка
$vote_id = 'vote_id_example'; // строка
$url_id = 'url_id_example'; // строка
$broadcast_id = 'broadcast_id_example'; // строка
$edit_key = 'edit_key_example'; // строка
$sso = 'sso_example'; // строка

try {
    $result = $apiInstance->deleteCommentVote($tenant_id, $comment_id, $vote_id, $url_id, $broadcast_id, $edit_key, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteCommentVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]