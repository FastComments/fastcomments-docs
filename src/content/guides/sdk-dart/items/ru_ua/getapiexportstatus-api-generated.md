## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| batchJobId | string | query | No |  |
| sso | string | query | No |  |

## Ответ

Возвращает: `ModerationExportStatusResponse`

## Пример

[inline-code-attrs-start title = 'Пример getApiExportStatus'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final batchJobId = batchJobId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getApiExportStatus(tenantId, GetApiExportStatusOptions(batchJobId: batchJobId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getApiExportStatus: $e\n');
}
[inline-code-end]