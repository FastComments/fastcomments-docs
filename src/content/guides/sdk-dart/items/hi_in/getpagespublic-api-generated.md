किसी टेनेंट के लिए पेजों की सूची लौटाता है। FChat डेस्कटॉप क्लाइंट द्वारा अपनी रूम सूची भरने के लिए उपयोग किया जाता है।
प्रत्येक पेज के लिए रिज़ॉल्व किए गए कस्टम कॉन्फ़िग पर `enableFChat` का true होना आवश्यक है।
SSO की आवश्यकता वाले पेजों को अनुरोध करने वाले उपयोगकर्ता के ग्रुप एक्सेस के आधार पर फ़िल्टर किया जाता है।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| cursor | string | query | नहीं | पिछले अनुरोध से `nextCursor` के रूप में लौटाया गया अपारदर्शी पेजिनेशन कर्सर। उसी `sortBy` से बंधा होता है। |
| limit | integer | query | नहीं | 1..200, डिफ़ॉल्ट 50 |
| q | string | query | नहीं | वैकल्पिक, केस-असंवेदनशील शीर्षक प्रीफ़िक्स फ़िल्टर। |
| sortBy | string | query | नहीं | सॉर्ट क्रम। `updatedAt` (डिफ़ॉल्ट, नवीनतम पहले), `commentCount` (सबसे अधिक कमेंट पहले), या `title` (वर्णानुक्रमिक)। |
| hasComments | boolean | query | नहीं | यदि true है, तो केवल कम से कम एक कमेंट वाले पेज लौटाएँ। |

## प्रतिक्रिया

लौटाता है: `GetPublicPagesResponse`

## उदाहरण

[inline-code-attrs-start title = 'getPagesPublic उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | पिछले अनुरोध से `nextCursor` के रूप में लौटाया गया अपारदर्शी पेजिनेशन कर्सर। उसी `sortBy` से बंधा होता है।
final limit = 56; // int | 1..200, डिफ़ॉल्ट 50
final q = q_example; // String | वैकल्पिक, केस-असंवेदनशील शीर्षक प्रीफ़िक्स फ़िल्टर।
final sortBy = ; // PagesSortBy | सॉर्ट क्रम। `updatedAt` (डिफ़ॉल्ट, नवीनतम पहले), `commentCount` (सबसे अधिक कमेंट पहले), या `title` (वर्णानुक्रमिक)।
final hasComments = true; // bool | यदि true है, तो केवल कम से कम एक कमेंट वाले पेज लौटाएँ।

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]