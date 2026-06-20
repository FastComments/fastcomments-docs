## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserInternalProfileResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserInternalProfile'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getUserInternalProfile($comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getUserInternalProfile: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]