## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| id | string | query | Yes |  |

## Одговор

Returns: `GetV2PageReactUsersResponse`

## Пример

[inline-code-attrs-start title = 'Primer getV2PageReactUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getV2PageReactUsers(tenantId, urlId, id);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getV2PageReactUsers: $e\n');
}
[inline-code-end]