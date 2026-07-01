Anfrage  
tenantId  
urlId  

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|-----|-----|--------------|--------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| page | integer | query | Nein |  |
| direction | string | query | Nein |  |
| sso | string | query | Nein |  |
| skip | integer | query | Nein |  |
| skipChildren | integer | query | Nein |  |
| limit | integer | query | Nein |  |
| limitChildren | integer | query | Nein |  |
| countChildren | boolean | query | Nein |  |
| fetchPageForCommentId | string | query | Nein |  |
| includeConfig | boolean | query | Nein |  |
| countAll | boolean | query | Nein |  |
| includei10n | boolean | query | Nein |  |
| locale | string | query | Nein |  |
| modules | string | query | Nein |  |
| isCrawler | boolean | query | Nein |  |
| includeNotificationCount | boolean | query | Nein |  |
| asTree | boolean | query | Nein |  |
| maxTreeDepth | integer | query | Nein |  |
| useFullTranslationIds | boolean | query | Nein |  |
| parentId | string | query | Nein |  |
| searchText | string | query | Nein |  |
| hashTags | array | query | Nein |  |
| userId | string | query | Nein |  |
| customConfigStr | string | query | Nein |  |
| afterCommentId | string | query | Nein |  |
| beforeCommentId | string | query | Nein |  |

## Antwort

Rückgabe: `GetCommentsPublicResponseWithPresencePublicComment`

## Beispiel

[inline-code-attrs-start title = 'getCommentsPublic Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final page = 56; // int | 
final direction = ; // SortDirections | 
final sso = sso_example; // String | 
final skip = 56; // int | 
final skipChildren = 56; // int | 
final limit = 56; // int | 
final limitChildren = 56; // int | 
final countChildren = true; // bool | 
final fetchPageForCommentId = fetchPageForCommentId_example; // String | 
final includeConfig = true; // bool | 
final countAll = true; // bool | 
final includei10n = true; // bool | 
final locale = locale_example; // String | 
final modules = modules_example; // String | 
final isCrawler = true; // bool | 
final includeNotificationCount = true; // bool | 
final asTree = true; // bool | 
final maxTreeDepth = 56; // int | 
final useFullTranslationIds = true; // bool | 
final parentId = parentId_example; // String | 
final searchText = searchText_example; // String | 
final hashTags = []; // List<String> | 
final userId = userId_example; // String | 
final customConfigStr = customConfigStr_example; // String | 
final afterCommentId = afterCommentId_example; // String | 
final beforeCommentId = beforeCommentId_example; // String | 

try {
    final result = api_instance.getCommentsPublic(tenantId, urlId, GetCommentsPublicOptions(page: page, direction: direction, sso: sso, skip: skip, skipChildren: skipChildren, limit: limit, limitChildren: limitChildren, countChildren: countChildren, fetchPageForCommentId: fetchPageForCommentId, includeConfig: includeConfig, countAll: countAll, includei10n: includei10n, locale: locale, modules: modules, isCrawler: isCrawler, includeNotificationCount: includeNotificationCount, asTree: asTree, maxTreeDepth: maxTreeDepth, useFullTranslationIds: useFullTranslationIds, parentId: parentId, searchText: searchText, hashTags: hashTags, userId: userId, customConfigStr: customConfigStr, afterCommentId: afterCommentId, beforeCommentId: beforeCommentId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getCommentsPublic: $e\n');
}
[inline-code-end]