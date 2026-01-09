## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| commentId | string | path | Так |  |
| dir | integer | query | Так |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentVoteUserNames200Response.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentVoteUserNames'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$dir = 56; // int
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getCommentVoteUserNames($tenant_id, $comment_id, $dir, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentVoteUserNames: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]