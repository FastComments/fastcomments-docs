## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantPackagesResponse.php)

## उदाहरण

[inline-code-attrs-start title = 'getTenantPackages उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$skip = 3.4; // float


try {
    $result = $apiInstance->getTenantPackages($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenantPackages: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]