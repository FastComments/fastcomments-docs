## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|------|-----|
| tenantId | string | query | 是 |  |
| page | integer | query | 否 |  |
| limit | integer | query | 否 |  |
| skip | integer | query | 否 |  |
| asTree | boolean | query | 否 |  |
| skipChildren | integer | query | 否 |  |
| limitChildren | integer | query | 否 |  |
| maxTreeDepth | integer | query | 否 |  |
| urlId | string | query | 否 |  |
| userId | string | query | 否 |  |
| anonUserId | string | query | 否 |  |
| contextUserId | string | query | 否 |  |
| hashTag | string | query | 否 |  |
| parentId | string | query | 否 |  |
| direction | string | query | 否 |  |
| fromDate | integer | query | 否 |  |
| toDate | integer | query | 否 |  |

## 回應

返回：`APIGetCommentsResponse`

## 範例

[inline-code-attrs-start title = 'getComments 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 金鑰授權：api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消註解下方以設定 API 金鑰的前綴（例如 Bearer），若需要
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