## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| commentId | string | path | हाँ |  |
| broadcastId | string | query | हाँ |  |
| editKey | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `PublicAPIDeleteCommentResponse`

## उदाहरण

[inline-code-attrs-start title = 'deleteCommentPublic उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final editKey = editKey_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.deleteCommentPublic(tenantId, commentId, broadcastId, DeleteCommentPublicOptions(editKey: editKey, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->deleteCommentPublic: $e\n');
}
[inline-code-end]