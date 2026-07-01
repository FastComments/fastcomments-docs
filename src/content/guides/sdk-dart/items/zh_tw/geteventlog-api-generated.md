req
tenantId
urlId
userIdWS

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| userIdWS | string | query | 是 |  |
| startTime | integer | query | 是 |  |
| endTime | integer | query | 否 |  |

## 回應

Returns: `GetEventLogResponse`

## 範例

[inline-code-attrs-start title = 'getEventLog 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // 字串 | 
final urlId = urlId_example; // 字串 | 
final userIdWS = userIdWS_example; // 字串 | 
final startTime = 789; // 整數 | 
final endTime = 789; // 整數 | 

try {
    final result = api_instance.getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getEventLog: $e\n');
}
[inline-code-end]

---