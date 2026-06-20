## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Nee |  |
| sso | string | query | Nee |  |

## Antwoord

Geeft terug: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationPageSearchResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'getSearchPages Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel; `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client()
);
$value = 'value_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getSearchPages($value, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchPages: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]