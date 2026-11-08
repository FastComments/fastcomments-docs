किसी टेनेंट के लिए पेजों की सूची लौटाता है। FChat डेस्कटॉप क्लाइंट द्वारा अपनी रूम सूची भरने के लिए उपयोग किया जाता है।
प्रत्येक पेज के रिज़ॉल्व किए गए कस्टम कॉन्फ़िग पर `enableFChat` true होना आवश्यक है।
जो पेज SSO की मांग करते हैं, उन्हें अनुरोध करने वाले उपयोगकर्ता के ग्रुप एक्सेस के आधार पर फ़िल्टर किया जाता है।

## पैरामीटर्स

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| cursor | string | query | नहीं | पिछले अनुरोध से `nextCursor` के रूप में लौटाया गया अपारदर्शी पेजिनेशन कर्सर। उसी `sortBy` से बंधा होता है। |
| limit | integer | query | नहीं | 1..200, डिफ़ॉल्ट 50 |
| q | string | query | नहीं | वैकल्पिक, केस-असंवेदनशील शीर्षक प्रीफ़िक्स फ़िल्टर। |
| sortBy | string | query | नहीं | सॉर्ट क्रम। `updatedAt` (डिफ़ॉल्ट, नवीनतम पहले), `commentCount` (सबसे अधिक कमेंट्स पहले), या `title` (वर्णानुक्रम)। |
| hasComments | boolean | query | नहीं | यदि true, तो केवल कम से कम एक कमेंट वाले पेज लौटाएं। |

## प्रतिक्रिया

लौटाता है: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## उदाहरण

[inline-code-attrs-start title = 'getPagesPublic उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // यदि आप कस्टम http क्लाइंट का उपयोग करना चाहते हैं, तो अपना क्लाइंट पास करें जो `GuzzleHttp\ClientInterface` को इम्प्लीमेंट करता हो।
    // यह वैकल्पिक है, `GuzzleHttp\Client` डिफ़ॉल्ट रूप से उपयोग किया जाएगा।
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | पिछले अनुरोध से `nextCursor` के रूप में लौटाया गया अपारदर्शी पेजिनेशन कर्सर। उसी `sortBy` से बंधा होता है।
    'limit' => 56, // int | 1..200, डिफ़ॉल्ट 50
    'q' => 'q_example', // string | वैकल्पिक, केस-असंवेदनशील शीर्षक प्रीफ़िक्स फ़िल्टर।
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | सॉर्ट क्रम। `updatedAt` (डिफ़ॉल्ट, नवीनतम पहले), `commentCount` (सबसे अधिक कमेंट्स पहले), या `title` (वर्णानुक्रम)।
    'has_comments' => True, // bool | यदि true, तो केवल कम से कम एक कमेंट वाले पेज लौटाएं।
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]