## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrača: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationUserSearchResponse.php)

## Primer

[inline-code-attrs-start title = 'getSearchUsers Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Če želite uporabiti prilagojen HTTP odjemalec, posredujte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, kot privzeti bo uporabljen `GuzzleHttp\Client`.
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

---