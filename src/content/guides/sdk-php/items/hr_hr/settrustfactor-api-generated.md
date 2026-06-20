## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Ne |  |
| trustFactor | string | query | Ne |  |
| sso | string | query | Ne |  |

## Response

Vraća: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SetUserTrustFactorResponse.php)

## Primjer

[inline-code-attrs-start title = 'setTrustFactor Primjer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ako želite koristiti prilagođeni HTTP klijent, proslijedite klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, kao zadani klijent će se koristiti `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$user_id = 'user_id_example'; // string
$trust_factor = 'trust_factor_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->setTrustFactor($user_id, $trust_factor, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->setTrustFactor: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]