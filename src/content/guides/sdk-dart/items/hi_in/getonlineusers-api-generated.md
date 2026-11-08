किसी पेज के वर्तमान में ऑनलाइन दर्शक: वे लोग जिनका websocket सत्र इस समय पेज की सदस्यता लिए हुए है।
anonCount + totalCount लौटाता है (पूरे रूम के सदस्य, जिनमें वे अनाम दर्शक भी शामिल हैं जिन्हें हम सूचीबद्ध नहीं करते)।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)। |
| afterName | string | query | नहीं | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें। |
| afterUserId | string | query | नहीं | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। आवश्यक है जब afterName सेट हो ताकि समान नाम वाली प्रविष्टियाँ न छूटें। |

## प्रतिक्रिया

लौटाता है: `PageUsersOnlineResponse`

## उदाहरण

[inline-code-attrs-start title = 'getOnlineUsers उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)।
final afterName = afterName_example; // String | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें।
final afterUserId = afterUserId_example; // String | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। आवश्यक है जब afterName सेट हो ताकि समान नाम वाली प्रविष्टियाँ न छूटें।

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]