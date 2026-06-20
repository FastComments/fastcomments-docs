## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| direction | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`VoteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример postVote'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это опционально, `GuzzleHttp\Client` будет использован как значение по умолчанию.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // строка
$direction = 'direction_example'; // строка
$sso = 'sso_example'; // строка

try {
    $result = $apiInstance->postVote($comment_id, $direction, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---