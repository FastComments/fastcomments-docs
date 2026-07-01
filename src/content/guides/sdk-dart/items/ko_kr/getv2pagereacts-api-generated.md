## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|------|------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |

## 응답

Returns: `GetV2PageReacts`

## 예시

[inline-code-attrs-start title = 'getV2PageReacts 예제'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // 문자열 | 
final urlId = urlId_example; // 문자열 | 

try {
    final result = api_instance.getV2PageReacts(tenantId, urlId);
    print(result);
} catch (e) {
    print('PublicApi->getV2PageReacts 호출 시 예외: $e\n');
}
[inline-code-end]