## Параметри

| Назва | Тип | Розташування | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| includeEmail | boolean | query | No |  |
| includeIP | boolean | query | No |  |
| sso | string | query | No |  |

## Відповідь

Повертає: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPICommentResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getModerationComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // рядок
$include_email = True; // булевий
$include_ip = True; // булевий
$sso = 'sso_example'; // рядок

try {
    $result = $apiInstance->getModerationComment($comment_id, $include_email, $include_ip, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getModerationComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]