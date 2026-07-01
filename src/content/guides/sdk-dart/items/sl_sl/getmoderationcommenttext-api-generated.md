## Parameters

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | poizvedba | Da |  |
| commentId | string | pot | Da |  |
| sso | string | poizvedba | Ne |  |

## Response

Vrne: `GetCommentTextResponse`

## Example

[inline-code-attrs-start title = 'getModerationCommentText Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getModerationCommentText(tenantId, commentId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getModerationCommentText: $e\n');
}
[inline-code-end]