Bulk 사용자 정보는 테넌트 단위로 제공됩니다. userIds가 주어지면 User / SSOUser의 표시 정보를 반환합니다.  
댓글 위젯에서 방금 존재 이벤트를 통해 나타난 사용자를 풍부하게 표시하는 데 사용됩니다.  
페이지 컨텍스트가 없으며: 프라이버시가 일관되게 적용됩니다(비공개 프로필은 마스킹됩니다).

## Parameters

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| ids | string | query | 예 | 쉼표로 구분된 userIds. |

## Response

반환: `PageUsersInfoResponse`

## 예시

[inline-code-attrs-start title = 'getUsersInfo 예제'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String |
final ids = ids_example; // String | 쉼표로 구분된 userIds.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]