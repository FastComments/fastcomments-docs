## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| approved | boolean | query | No |  |
| sso | string | query | No |  |

## Відповідь

Повертає: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SetCommentApprovedResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад postSetCommentApprovalStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використовуватись `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$approved = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postSetCommentApprovalStatus($comment_id, $approved, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postSetCommentApprovalStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]