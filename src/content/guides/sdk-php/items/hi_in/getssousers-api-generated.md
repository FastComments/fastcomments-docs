## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| skip | integer | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetSSOUsersResponse.php)

## उदाहरण

[inline-code-attrs-start title = 'getSSOUsers उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API key authorization कॉन्फ़िगर करें: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// यदि आवश्यक हो तो API key के लिए prefix (जैसे Bearer) सेटअप करने के लिए नीचे को अनकमेंट करें
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // यदि आप कस्टम http client का उपयोग करना चाहते हैं, तो अपना client पास करें जो `GuzzleHttp\ClientInterface` को इम्प्लीमेंट करता हो।
    // यह वैकल्पिक है, `GuzzleHttp\Client` डिफ़ॉल्ट रूप से उपयोग किया जाएगा।
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$skip = 56; // int


try {
    $result = $apiInstance->getSSOUsers($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'DefaultApi->getSSOUsers कॉल करते समय Exception: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]