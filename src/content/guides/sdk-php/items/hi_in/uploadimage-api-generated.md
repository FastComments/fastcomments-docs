छवि अपलोड और रिसाइज़ करें

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| sizePreset | string | query | नहीं | साइज़ प्रीसेट: "Default" (1000x1000px) या "CrossPlatform" (लोकप्रिय डिवाइसों के लिए साइज़ बनाता है) |
| urlId | string | query | नहीं | पेज id जिससे अपलोड हो रहा है, कॉन्फ़िगर करने के लिए |

## प्रतिक्रिया

रिटर्न करता है: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## उदाहरण

[inline-code-attrs-start title = 'uploadImage उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // यदि आप कस्टम http क्लाइंट का उपयोग करना चाहते हैं, तो अपना क्लाइंट पास करें जो `GuzzleHttp\ClientInterface` को इम्प्लीमेंट करता हो।
    // यह वैकल्पिक है, `GuzzleHttp\Client` डिफ़ॉल्ट रूप से उपयोग किया जाएगा।
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$file = '/path/to/file.txt'; // \SplFileObject
$options = [
    'size_preset' => new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(), // \FastComments\Client\Model\SizePreset | साइज़ प्रीसेट: \"Default\" (1000x1000px) या \"CrossPlatform\" (लोकप्रिय डिवाइसों के लिए साइज़ बनाता है)
    'url_id' => 'url_id_example', // string | पेज id जिससे अपलोड हो रहा है, कॉन्फ़िगर करने के लिए
];


try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]