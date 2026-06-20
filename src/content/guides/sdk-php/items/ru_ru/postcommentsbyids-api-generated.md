## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIChildCommentsResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример postCommentsByIds'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Если вы хотите использовать собственный HTTP-клиент, передайте клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$comments_by_ids_params = new \FastComments\Client\Model\CommentsByIdsParams(); // \FastComments\Client\Model\CommentsByIdsParams
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postCommentsByIds($comments_by_ids_params, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postCommentsByIds: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]