## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| postId | string | path | Ja |  |
| isUndo | boolean | query | Nee |  |
| broadcastId | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: `ReactFeedPostResponse`

## Voorbeeld

[inline-code-attrs-start title = 'reactFeedPostPublic Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final postId = postId_example; // String | 
final reactBodyParams = ReactBodyParams(); // ReactBodyParams | 
final isUndo = true; // bool | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.reactFeedPostPublic(tenantId, postId, reactBodyParams, ReactFeedPostPublicOptions(isUndo: isUndo, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->reactFeedPostPublic: $e\n');
}
[inline-code-end]

---