## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetBannedUsersCountResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getCounts'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ako želite da koristite prilagođeni HTTP klijent, prosledite klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, kao podrazumevani će biti korišćen `GuzzleHttp\Client`.
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