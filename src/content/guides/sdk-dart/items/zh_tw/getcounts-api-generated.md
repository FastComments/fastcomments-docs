## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| sso | string | query | 否 |  |

## 回應

返回：`GetBannedUsersCountResponse`

## 範例

[inline-code-attrs-start title = 'getCounts 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getCounts(tenantId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getCounts: $e\n');
}
[inline-code-end]