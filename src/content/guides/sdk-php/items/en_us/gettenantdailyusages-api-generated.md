## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| yearNumber | number | query | No |  |
| monthNumber | number | query | No |  |
| dayNumber | number | query | No |  |
| skip | number | query | No |  |

## Response

Returns: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantDailyUsages200Response.php)

## Example

[inline-code-attrs-start title = 'getTenantDailyUsages Example'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to set up prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want to use a custom HTTP client, pass a client which implements `GuzzleHttp\ClientInterface`.
    // This is optional; `GuzzleHttp\Client` will be used by default.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$year_number = 3.4; // float
$month_number = 3.4; // float
$day_number = 3.4; // float
$skip = 3.4; // float

try {
    $result = $apiInstance->getTenantDailyUsages($tenant_id, $year_number, $month_number, $day_number, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenantDailyUsages: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]