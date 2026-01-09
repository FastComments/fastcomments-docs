## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| broadcastId | string | query | Yes |  |
| sessionId | string | query | No |  |
| sso | string | query | No |  |

## Ответ

Возвращает: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateCommentPublic200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример createCommentPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // строка
$url_id = 'url_id_example'; // строка
$broadcast_id = 'broadcast_id_example'; // строка
$comment_data = new \FastComments\Client\Model\CommentData(); // \FastComments\Client\Model\CommentData
$session_id = 'session_id_example'; // строка
$sso = 'sso_example'; // строка

try {
    $result = $apiInstance->createCommentPublic($tenant_id, $url_id, $broadcast_id, $comment_data, $session_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->createCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]