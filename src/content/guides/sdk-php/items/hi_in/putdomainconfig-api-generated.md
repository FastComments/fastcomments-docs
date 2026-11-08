## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| domainToUpdate | string | path | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PutDomainConfigResponse.php)

## उदाहरण

[inline-code-attrs-start title = 'putDomainConfig उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API की ऑथराइज़ेशन कॉन्फ़िगर करें: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// यदि आवश्यक हो तो API की के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे को अनकमेंट करें
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // यदि आप कस्टम http क्लाइंट का उपयोग करना चाहते हैं, तो अपना क्लाइंट पास करें जो `GuzzleHttp\ClientInterface` इम्प्लीमेंट करता हो।
    // यह वैकल्पिक है, `GuzzleHttp\Client` डिफ़ॉल्ट रूप से उपयोग किया जाएगा।
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$domain_to_update = 'domain_to_update_example'; // string
$update_domain_config_params = new \FastComments\Client\Model\UpdateDomainConfigParams(); // \FastComments\Client\Model\UpdateDomainConfigParams


try {
    $result = $apiInstance->putDomainConfig($tenant_id, $domain_to_update, $update_domain_config_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->putDomainConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]