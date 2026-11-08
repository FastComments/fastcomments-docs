## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| broadcastId | string | query | हाँ |  |
| sessionId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `SaveCommentsResponseWithPresence`

## उदाहरण

[inline-code-attrs-start title = 'createCommentPublic उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final commentData = CommentData(); // CommentData | 
final sessionId = sessionId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.createCommentPublic(tenantId, urlId, broadcastId, commentData, CreateCommentPublicOptions(sessionId: sessionId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->createCommentPublic: $e\n');
}
[inline-code-end]