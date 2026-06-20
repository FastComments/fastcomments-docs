## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| direction | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`VoteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteResponse.php)

## Пример

[inline-code-attrs-start title = 'postVote Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционо, `GuzzleHttp\Client` ће бити коришћен као подразумевани.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // низ
$direction = 'direction_example'; // низ
$sso = 'sso_example'; // низ

try {
    $result = $apiInstance->postVote($comment_id, $direction, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]