## Ответ

Возвращает: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'logoutPublic Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();

try {
    final result = api_instance.logoutPublic();
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->logoutPublic: $e\n');
}
[inline-code-end]