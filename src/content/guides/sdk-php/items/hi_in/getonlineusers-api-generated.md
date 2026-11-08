किसी पेज के वर्तमान में-ऑनलाइन दर्शक: वे लोग जिनका websocket सत्र इस समय पेज की सदस्यता लिए हुए है।
anonCount + totalCount लौटाता है (पूरे रूम के सदस्य, जिनमें वे अनाम दर्शक भी शामिल हैं जिन्हें हम सूचीबद्ध नहीं करते)।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)। |
| afterName | string | query | नहीं | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें। |
| afterUserId | string | query | नहीं | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। आवश्यक जब afterName सेट हो ताकि समान नाम वाली प्रविष्टियाँ न छूटें। |

## प्रतिक्रिया

लौटाता है: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## उदाहरण

[inline-code-attrs-start title = 'getOnlineUsers उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // यदि आप कस्टम http क्लाइंट का उपयोग करना चाहते हैं, तो अपना क्लाइंट पास करें जो `GuzzleHttp\ClientInterface` को लागू करता है।
    // यह वैकल्पिक है, डिफ़ॉल्ट रूप से `GuzzleHttp\Client` का उपयोग किया जाएगा।
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)।
$options = [
    'after_name' => 'after_name_example', // string | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें।
    'after_user_id' => 'after_user_id_example', // string | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। आवश्यक जब afterName सेट हो ताकि समान नाम वाली प्रविष्टियाँ न छूटें।
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]