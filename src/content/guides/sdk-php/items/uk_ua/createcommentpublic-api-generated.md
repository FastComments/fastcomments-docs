## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |
| broadcastId | string | query | Так |  |
| sessionId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateCommentPublic200Response.php)

## Приклад

[inline-code-attrs-start title = 'Приклад createCommentPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використовуватись `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$comment_data = new \FastComments\Client\Model\CommentData(); // \FastComments\Client\Model\CommentData
$session_id = 'session_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->createCommentPublic($tenant_id, $url_id, $broadcast_id, $comment_data, $session_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->createCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]