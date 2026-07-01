req
tenantId
urlId

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| page | integer | query | Non |  |
| direction | string | query | Non |  |
| sso | string | query | Non |  |
| skip | integer | query | Non |  |
| skipChildren | integer | query | Non |  |
| limit | integer | query | Non |  |
| limitChildren | integer | query | Non |  |
| countChildren | boolean | query | Non |  |
| fetchPageForCommentId | string | query | Non |  |
| includeConfig | boolean | query | Non |  |
| countAll | boolean | query | Non |  |
| includei10n | boolean | query | Non |  |
| locale | string | query | Non |  |
| modules | string | query | Non |  |
| isCrawler | boolean | query | Non |  |
| includeNotificationCount | boolean | query | Non |  |
| asTree | boolean | query | Non |  |
| maxTreeDepth | integer | query | Non |  |
| useFullTranslationIds | boolean | query | Non |  |
| parentId | string | query | Non |  |
| searchText | string | query | Non |  |
| hashTags | array | query | Non |  |
| userId | string | query | Non |  |
| customConfigStr | string | query | Non |  |
| afterCommentId | string | query | Non |  |
| beforeCommentId | string | query | Non |  |

## Réponse

Renvoie : `GetCommentsResponseWithPresencePublicComment`

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentsPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
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