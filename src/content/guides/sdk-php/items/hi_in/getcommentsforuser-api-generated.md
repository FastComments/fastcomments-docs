## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| userId | string | query | नहीं |  |
| direction | string | query | नहीं |  |
| repliesToUserId | string | query | नहीं |  |
| page | number | query | नहीं |  |
| includei10n | boolean | query | नहीं |  |
| locale | string | query | नहीं |  |
| isCrawler | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsForUserResponse.php)

## उदाहरण

[inline-code-attrs-start title = 'getCommentsForUser उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // यदि आप कस्टम http क्लाइंट का उपयोग करना चाहते हैं, तो अपना क्लाइंट पास करें जो `GuzzleHttp\ClientInterface` को इम्प्लीमेंट करता हो।
    // यह वैकल्पिक है, `GuzzleHttp\Client` डिफ़ॉल्ट रूप से उपयोग किया जाएगा।
    new GuzzleHttp\Client()
);

$options = [
    'user_id' => 'user_id_example', // string
    'direction' => new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(), // \FastComments\Client\Model\SortDirections
    'replies_to_user_id' => 'replies_to_user_id_example', // string
    'page' => 3.4, // float
    'includei10n' => True, // bool
    'locale' => 'locale_example', // string
    'is_crawler' => True, // bool
];


try {
    $result = $apiInstance->getCommentsForUser($options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsForUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]