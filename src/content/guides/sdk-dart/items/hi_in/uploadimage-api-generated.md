एक इमेज अपलोड और रिसाइज़ करें

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| sizePreset | string | query | नहीं | साइज़ प्रीसेट: "Default" (1000x1000px) या "CrossPlatform" (लोकप्रिय डिवाइसों के लिए साइज़ बनाता है) |
| urlId | string | query | नहीं | पेज id जिससे अपलोड हो रहा है, कॉन्फ़िगर करने के लिए |

## प्रतिक्रिया

रिटर्न करता है: `UploadImageResponse`

## उदाहरण

[inline-code-attrs-start title = 'uploadImage उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | साइज़ प्रीसेट: \"Default\" (1000x1000px) या \"CrossPlatform\" (लोकप्रिय डिवाइसों के लिए साइज़ बनाता है)
final urlId = urlId_example; // String | पेज id जिससे अपलोड हो रहा है, कॉन्फ़िगर करने के लिए

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]