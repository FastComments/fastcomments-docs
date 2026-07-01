## 參數

| 名稱 | 類型 | 位置 | 必需 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 是 |  |
| sso | string | query | 否 |  |

## 回應

返回: `APIEmptyResponse`

## 範例

[inline-code-attrs-start title = 'putReopenThread 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.putReopenThread(tenantId, urlId, sso);
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->putReopenThread: $e\n');
}
[inline-code-end]