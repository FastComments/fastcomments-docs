## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| usernameStartsWith | string | query | नहीं |  |
| mentionGroupIds | array | query | नहीं |  |
| sso | string | query | नहीं |  |
| searchSection | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SearchUsersResult.php)

## उदाहरण

[inline-code-attrs-start title = 'searchUsers उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // यदि आप कस्टम http क्लाइंट का उपयोग करना चाहते हैं, तो अपना क्लाइंट पास करें जो `GuzzleHttp\ClientInterface` को इम्प्लीमेंट करता हो।
    // यह वैकल्पिक है, डिफ़ॉल्ट रूप से `GuzzleHttp\Client` का उपयोग किया जाएगा।
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$options = [
    'username_starts_with' => 'username_starts_with_example', // string
    'mention_group_ids' => array('mention_group_ids_example'), // string[]
    'sso' => 'sso_example', // string
    'search_section' => 'search_section_example', // string
];


try {
    $result = $apiInstance->searchUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->searchUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]