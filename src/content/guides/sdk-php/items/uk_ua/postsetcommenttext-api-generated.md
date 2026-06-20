## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | шлях | Yes |  |
| sso | string | параметр запиту | No |  |

## Відповідь

Повертає: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SetCommentTextResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад використання postSetCommentText'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$set_comment_text_params = new \FastComments\Client\Model\SetCommentTextParams(); // \FastComments\Client\Model\SetCommentTextParams
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postSetCommentText($comment_id, $set_comment_text_params, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postSetCommentText: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---