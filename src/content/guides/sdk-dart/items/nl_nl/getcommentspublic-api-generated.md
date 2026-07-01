req
tenantId
urlId

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| page | integer | query | Nee |  |
| direction | string | query | Nee |  |
| sso | string | query | Nee |  |
| skip | integer | query | Nee |  |
| skipChildren | integer | query | Nee |  |
| limit | integer | query | Nee |  |
| limitChildren | integer | query | Nee |  |
| countChildren | boolean | query | Nee |  |
| fetchPageForCommentId | string | query | Nee |  |
| includeConfig | boolean | query | Nee |  |
| countAll | boolean | query | Nee |  |
| includei10n | boolean | query | Nee |  |
| locale | string | query | Nee |  |
| modules | string | query | Nee |  |
| isCrawler | boolean | query | Nee |  |
| includeNotificationCount | boolean | query | Nee |  |
| asTree | boolean | query | Nee |  |
| maxTreeDepth | integer | query | Nee |  |
| useFullTranslationIds | boolean | query | Nee |  |
| parentId | string | query | Nee |  |
| searchText | string | query | Nee |  |
| hashTags | array | query | Nee |  |
| userId | string | query | Nee |  |
| customConfigStr | string | query | Nee |  |
| afterCommentId | string | query | Nee |  |
| beforeCommentId | string | query | Nee |  |

## Respons

Retourneert: `GetCommentsResponseWithPresencePublicComment`

## Voorbeeld

[inline-code-attrs-start title = 'getCommentsPublic Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
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