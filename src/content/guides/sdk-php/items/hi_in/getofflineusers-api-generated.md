पेज पर पिछले टिप्पणीकार जो अभी ऑनलाइन नहीं हैं। displayName के अनुसार क्रमबद्ध।
"Members" सेक्शन रेंडर करने के लिए /users/online का उपयोग पूरा करने के बाद इसका उपयोग करें।
commenterName पर कर्सर पेजिनेशन: सर्वर afterName से आगे $gt के माध्यम से आंशिक {tenantId, urlId, commenterName}
इंडेक्स पर चलता है, कोई $skip लागत नहीं।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)। |
| afterName | string | query | नहीं | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें। |
| afterUserId | string | query | नहीं | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। afterName सेट होने पर आवश्यक ताकि नाम-टाई में प्रविष्टियाँ न छूटें। |

## प्रतिक्रिया

रिटर्न करता है: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## उदाहरण

[inline-code-attrs-start title = 'getOfflineUsers उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // यदि आप कस्टम http क्लाइंट का उपयोग करना चाहते हैं, तो अपना क्लाइंट पास करें जो `GuzzleHttp\ClientInterface` लागू करता है।
    // यह वैकल्पिक है, `GuzzleHttp\Client` डिफ़ॉल्ट रूप से उपयोग किया जाएगा।
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)।
$options = [
    'after_name' => 'after_name_example', // string | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें।
    'after_user_id' => 'after_user_id_example', // string | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। afterName सेट होने पर आवश्यक ताकि नाम-टाई में प्रविष्टियाँ न छूटें।
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]