## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| urlId | string | query | नहीं |  |
| fromCommentId | string | query | नहीं |  |
| viewed | boolean | query | नहीं |  |
| type | string | query | नहीं |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetNotificationsResponse.php)

## उदाहरण

[inline-code-attrs-start title = 'getNotifications उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API की अधिकृत अधिकारण कॉन्फ़िगर करें: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// यदि आवश्यक हो तो API की के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे को अनकमेंट करें
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // यदि आप कस्टम http क्लाइंट का उपयोग करना चाहते हैं, तो अपना क्लाइंट पास करें जो `GuzzleHttp\ClientInterface` लागू करता है।
    // यह वैकल्पिक है, डिफ़ॉल्ट रूप से `GuzzleHttp\Client` का उपयोग किया जाएगा।
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'user_id' => 'user_id_example', // string
    'url_id' => 'url_id_example', // string
    'from_comment_id' => 'from_comment_id_example', // string
    'viewed' => True, // bool
    'type' => 'type_example', // string
    'skip' => 3.4, // float
];


try {
    $result = $apiInstance->getNotifications($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]