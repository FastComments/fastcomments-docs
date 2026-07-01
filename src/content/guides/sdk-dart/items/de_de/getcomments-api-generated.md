## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | integer | query | Nein |  |
| limit | integer | query | Nein |  |
| skip | integer | query | Nein |  |
| asTree | boolean | query | Nein |  |
| skipChildren | integer | query | Nein |  |
| limitChildren | integer | query | Nein |  |
| maxTreeDepth | integer | query | Nein |  |
| urlId | string | query | Nein |  |
| userId | string | query | Nein |  |
| anonUserId | string | query | Nein |  |
| contextUserId | string | query | Nein |  |
| hashTag | string | query | Nein |  |
| parentId | string | query | Nein |  |
| direction | string | query | Nein |  |
| fromDate | integer | query | Nein |  |
| toDate | integer | query | Nein |  |

## Antwort

Returns: `APIGetCommentsResponse`

## Beispiel

[inline-code-attrs-start title = 'getComments Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurieren Sie die API-Schlüssel-Authentifizierung: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Entfernen Sie das Kommentarzeichen unten, um das Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls erforderlich
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final page = 56; // int | 
final limit = 56; // int | 
final skip = 56; // int | 
final asTree = true; // bool | 
final skipChildren = 56; // int | 
final limitChildren = 56; // int | 
final maxTreeDepth = 56; // int | 
final urlId = urlId_example; // String | 
final userId = userId_example; // String | 
final anonUserId = anonUserId_example; // String | 
final contextUserId = contextUserId_example; // String | 
final hashTag = hashTag_example; // String | 
final parentId = parentId_example; // String | 
final direction = ; // SortDirections | 
final fromDate = 789; // int | 
final toDate = 789; // int | 

try {
    final result = api_instance.getComments(tenantId, GetCommentsOptions(page: page, limit: limit, skip: skip, asTree: asTree, skipChildren: skipChildren, limitChildren: limitChildren, maxTreeDepth: maxTreeDepth, urlId: urlId, userId: userId, anonUserId: anonUserId, contextUserId: contextUserId, hashTag: hashTag, parentId: parentId, direction: direction, fromDate: fromDate, toDate: toDate));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getComments: $e\n');
}
[inline-code-end]