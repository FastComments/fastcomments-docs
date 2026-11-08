## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ResetUserNotificationsResponse.php)

## उदाहरण

[inline-code-attrs-start title = 'resetUserNotificationCount उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // यदि आप कस्टम http क्लाइंट का उपयोग करना चाहते हैं, तो अपना क्लाइंट पास करें जो `GuzzleHttp\ClientInterface` को इम्प्लीमेंट करता है।
    // यह वैकल्पिक है, डिफ़ॉल्ट रूप से `GuzzleHttp\Client` का उपयोग किया जाएगा।
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$sso = 'sso_example'; // string


try {
    $result = $apiInstance->resetUserNotificationCount($tenant_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->resetUserNotificationCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]