## Parameters

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| dir | integer | query | Yes |  |
| sso | string | query | No |  |

## Відповідь

Повертає: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentVoteUserNamesSuccessResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentVoteUserNames'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використати власний HTTP-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це опціонально, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // рядок
$comment_id = 'comment_id_example'; // рядок
$dir = 56; // int
$sso = 'sso_example'; // рядок


try {
    $result = $apiInstance->getCommentVoteUserNames($tenant_id, $comment_id, $dir, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentVoteUserNames: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]