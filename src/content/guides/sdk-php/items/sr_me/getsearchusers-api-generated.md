## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| value | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationUserSearchResponse.php)

## Примјер

[inline-code-attrs-start title = 'getSearchUsers Примјер'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако желите да користите прилагођен HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, `GuzzleHttp\Client` ће бити коришћен као подразумевани.
    new GuzzleHttp\Client()
);
$value = 'value_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getSearchUsers($value, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]