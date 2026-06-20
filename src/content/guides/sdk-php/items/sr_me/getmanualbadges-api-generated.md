## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| sso | string | query | Не |  |

## Одговор

Враћа: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantManualBadgesResponse.php)

## Пример

[inline-code-attrs-start title = 'getManualBadges Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционо, подразумевано ће бити коришћен `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getManualBadges($sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getManualBadges: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---