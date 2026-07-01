Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations.  
Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| parentTenantId | string | query | 아니오 |  |
| includeStats | boolean | query | 아니오 |  |

## 응답

반환: `AggregateResponse`

## 예시

[inline-code-attrs-start title = '집계 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API 키 인증 구성: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 아래 주석을 해제하여 API 키에 대한 접두사 (예: Bearer)를 설정합니다, 필요 시
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final aggregationRequest = AggregationRequest(); // AggregationRequest | 
final parentTenantId = parentTenantId_example; // String | 
final includeStats = true; // bool | 

try {
    final result = api_instance.aggregate(tenantId, aggregationRequest, AggregateOptions(parentTenantId: parentTenantId, includeStats: includeStats));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->aggregate: $e\n');
}
[inline-code-end]

---