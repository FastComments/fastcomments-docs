req  
tenantId  
urlId  
userIdWS  

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい |  |
| userIdWS | string | query | はい |  |
| startTime | integer | query | はい |  |
| endTime | integer | query | いいえ |  |

## レスポンス

返却: `GetEventLogResponse`

## 例

[inline-code-attrs-start title = 'getEventLog の例'; type = ''; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final userIdWS = userIdWS_example; // String | 
final startTime = 789; // int | 
final endTime = 789; // int | 

try {
    final result = api_instance.getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getEventLog: $e\n');
}
[inline-code-end]