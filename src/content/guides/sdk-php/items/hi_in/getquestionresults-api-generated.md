## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| urlId | string | query | नहीं |  |
| userId | string | query | नहीं |  |
| startDate | string | query | नहीं |  |
| questionId | string | query | नहीं |  |
| questionIds | string | query | नहीं |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionResultsResponse.php)

## उदाहरण

[inline-code-attrs-start title = 'getQuestionResults उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API की अधिकृत करना: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// यदि आवश्यक हो तो API की के लिए प्रीफिक्स (जैसे Bearer) सेटअप करने के लिए नीचे को अनकमेंट करें
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // यदि आप कस्टम http क्लाइंट का उपयोग करना चाहते हैं, तो अपना क्लाइंट पास करें जो `GuzzleHttp\ClientInterface` को इम्प्लीमेंट करता है।
    // यह वैकल्पिक है, `GuzzleHttp\Client` डिफ़ॉल्ट रूप से उपयोग किया जाएगा।
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'url_id' => 'url_id_example', // string
    'user_id' => 'user_id_example', // string
    'start_date' => 'start_date_example', // string
    'question_id' => 'question_id_example', // string
    'question_ids' => 'question_ids_example', // string
    'skip' => 3.4, // float
];


try {
    $result = $apiInstance->getQuestionResults($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]