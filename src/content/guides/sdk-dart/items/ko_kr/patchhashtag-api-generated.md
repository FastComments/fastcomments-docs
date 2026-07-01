## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| tag | string | path | Yes |  |

## 응답

반환: `UpdateHashTagResponse`

## 예시

[inline-code-attrs-start title = 'patchHashTag 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API 키 인증 설정: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 아래 코멘트를 해제하여 (예: Bearer)와 같은 API 키 접두사를 설정하십시오, 필요 시
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final tag = tag_example; // String | 
final updateHashTagBody = UpdateHashTagBody(); // UpdateHashTagBody | 

try {
    final result = api_instance.patchHashTag(tenantId, tag, updateHashTagBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchHashTag: $e\n');
}
[inline-code-end]

---