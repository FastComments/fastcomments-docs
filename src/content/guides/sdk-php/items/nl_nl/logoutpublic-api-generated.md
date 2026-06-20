## Antwoord

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'logoutPublic Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client()
);

try {
    $result = $apiInstance->logoutPublic();
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->logoutPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---