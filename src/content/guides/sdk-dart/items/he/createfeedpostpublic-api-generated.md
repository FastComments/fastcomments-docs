## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## תגובה

Returns: `CreateFeedPostResponse`

## דוגמה

[inline-code-attrs-start title = 'createFeedPostPublic דוגמה'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final createFeedPostParams = CreateFeedPostParams(); // CreateFeedPostParams | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.createFeedPostPublic(tenantId, createFeedPostParams, CreateFeedPostPublicOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->createFeedPostPublic: $e\n');
}
[inline-code-end]