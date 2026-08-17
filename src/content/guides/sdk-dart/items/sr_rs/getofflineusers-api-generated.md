Претходни коментатори на страници који НИСУ тренутно онлајн. Сортирани по displayName.  
Користите ово након што исцрпите /users/online да прикажете одељак „Чланови“.  
Курсорска пагинација на commenterName: сервер пролази кроз парцијални {tenantId, urlId, commenterName}  
индекс од afterName напред преко $gt, без трошка $skip.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | No | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Тијебрејкер курсора: проследите nextAfterUserId из претходног одговора. Потребно када је afterName постављен како би се у случају везаних имена (name-ties) не изгубили уноси. |

## Response

Returns: `PageUsersOfflineResponse`

## Example

[inline-code-attrs-start title = 'getOfflineUsers Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Идентификатор URL странице (очишћен на серверу).
final afterName = afterName_example; // String | Курсор: проследите nextAfterName из претходног одговора.
final afterUserId = afterUserId_example; // String | Тијебрејкер курсора: проследите nextAfterUserId из претходног одговора. Потребно када је afterName постављен како би се у случају везаних имена (name-ties) не изгубили уноси.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]

---