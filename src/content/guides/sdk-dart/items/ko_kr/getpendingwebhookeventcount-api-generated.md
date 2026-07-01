## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| commentId | string | query | 아니오 |  |
| externalId | string | query | 아니오 |  |
| eventType | string | query | 아니오 |  |
| type | string | query | 아니오 |  |
| domain | string | query | 아니오 |  |
| attemptCountGT | number | query | 아니오 |  |

## 응답

반환: `GetPendingWebhookEventCountResponse`

## Example

[inline-code-attrs-start title = 'getPendingWebhookEventCount 예제'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API 키 인증 구성: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 아래의 주석을 해제하여 API 키에 대한 접두사 (예: Bearer)를 설정합니다, 필요하다면
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final externalId = externalId_example; // String | 
final eventType = eventType_example; // String | 
final type = type_example; // String | 
final domain = domain_example; // String | 
final attemptCountGT = 1.2; // double | 

try {
    final result = api_instance.getPendingWebhookEventCount(tenantId, GetPendingWebhookEventCountOptions(commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getPendingWebhookEventCount: $e\n');
}
[inline-code-end]