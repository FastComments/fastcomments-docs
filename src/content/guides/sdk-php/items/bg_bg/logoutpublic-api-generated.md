## Response

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Example

[inline-code-attrs-start title = 'Пример за logoutPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате персонализиран http клиент, предайте вашия клиент, който реализира `GuzzleHttp\ClientInterface`.
    // Това е по избор, `GuzzleHttp\Client` ще се използва по подразбиране.
    new GuzzleHttp\Client()
);

try {
    $result = $apiInstance->logoutPublic();
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->logoutPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]