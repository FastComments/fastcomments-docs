## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| commentId | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserInternalProfileResponse.php)

## Пример

[inline-code-attrs-start title = 'getUserInternalProfile Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите свој клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, као подразумевано ће се користити `GuzzleHttp\Client`.
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