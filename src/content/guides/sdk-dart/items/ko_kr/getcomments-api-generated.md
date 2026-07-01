## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| page | integer | query | 아니오 |  |
| limit | integer | query | 아니오 |  |
| skip | integer | query | 아니오 |  |
| asTree | boolean | query | 아니오 |  |
| skipChildren | integer | query | 아니오 |  |
| limitChildren | integer | query | 아니오 |  |
| maxTreeDepth | integer | query | 아니오 |  |
| urlId | string | query | 아니오 |  |
| userId | string | query | 아니오 |  |
| anonUserId | string | query | 아니오 |  |
| contextUserId | string | query | 아니오 |  |
| hashTag | string | query | 아니오 |  |
| parentId | string | query | 아니오 |  |
| direction | string | query | 아니오 |  |
| fromDate | integer | query | 아니오 |  |
| toDate | integer | query | 아니오 |  |

## 응답

반환: `APIGetCommentsResponse`

## 예시

[inline-code-attrs-start title = 'getComments 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API 키 인증 구성: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 아래를 주석 해제하여 API 키에 대한 접두사(예: Bearer)를 설정합니다(필요한 경우)
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