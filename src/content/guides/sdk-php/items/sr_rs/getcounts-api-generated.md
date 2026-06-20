## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| sso | string | query | Не |  |

## Одговор

Враћа: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetBannedUsersCountResponse.php)

## Пример

[inline-code-attrs-start title = 'getCounts пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако желите да користите прилагођени http клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, `GuzzleHttp\Client` ће бити коришћен подразумевано.
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getCounts($sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCounts: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---