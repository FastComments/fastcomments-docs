## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | path | Ja |  |
| postId | string | path | Ja |  |
| broadcastId | string | query | Nee |  |
| sso | string | query | Nee |  |

## Reactie

Retourneert: `CreateFeedPostResponse`

## Voorbeeld

[inline-code-attrs-start title = 'updateFeedPostPublic Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final postId = postId_example; // String | 
final updateFeedPostParams = UpdateFeedPostParams(); // UpdateFeedPostParams | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.updateFeedPostPublic(tenantId, postId, updateFeedPostParams, UpdateFeedPostPublicOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->updateFeedPostPublic: $e\n');
}
[inline-code-end]

---