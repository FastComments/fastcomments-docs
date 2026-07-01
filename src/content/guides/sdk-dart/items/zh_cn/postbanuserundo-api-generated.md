## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| sso | string | query | 否 |  |

## 响应

返回：`APIEmptyResponse`

## 示例

[inline-code-attrs-start title = 'postBanUserUndo 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final banUserUndoParams = BanUserUndoParams(); // BanUserUndoParams | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postBanUserUndo(tenantId, banUserUndoParams, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postBanUserUndo: $e\n');
}
[inline-code-end]