## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## 응답

반환: `GetModeratorResponse`

## 예시

[inline-code-attrs-start title = 'getModerator 예제'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API 키 인증 구성: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 아래의 주석을 해제하여 API 키에 대한 접두사 (예: Bearer)를 설정합니다, 필요 시
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // 문자열 |
final id = id_example; // 문자열 | 

try {
    final result = api_instance.getModerator(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getModerator: $e\n');
}
[inline-code-end]