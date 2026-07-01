## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## 응답

반환값: `PatchPageAPIResponse`

## 예시

[inline-code-attrs-start title = 'patchPage 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API 키 인증 구성: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 아래를 주석 해제하여 필요 시 API 키에 대한 접두사(예: Bearer)를 설정합니다
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateAPIPageData = UpdateAPIPageData(); // UpdateAPIPageData | 

try {
    final result = api_instance.patchPage(tenantId, id, updateAPIPageData);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchPage: $e\n');
}
[inline-code-end]