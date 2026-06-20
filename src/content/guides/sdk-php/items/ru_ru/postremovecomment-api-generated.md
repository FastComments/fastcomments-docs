## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PostRemoveCommentResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример postRemoveComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postRemoveComment($comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postRemoveComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]