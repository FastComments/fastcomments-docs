```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

Помощники SSO включены в пакет (`package:fastcomments_dart/sso/...`).

Клиент предоставляет три класса API:

- `DefaultApi` — методы, аутентифицируемые с помощью API‑ключа, для использования на сервере.
- `PublicApi` — публичные методы, не требующие API‑ключа, безопасные для браузеров и мобильных клиентов.
- `ModerationApi` — обширный набор живых и быстрых API модерации. Каждый метод `ModerationApi` принимает параметр `sso` и может аутентифицироваться через SSO или сессионный cookie FastComments.com.

```dart
import 'package:fastcomments_dart/api.dart';

final api = PublicApi(ApiClient(basePath: 'https://fastcomments.com'));
final comments = await api.getCommentsPublic('YOUR_TENANT_ID', 'YOUR_URL_ID');
```

```dart
import 'package:fastcomments_dart/api.dart';

final publicApi = PublicApi(ApiClient(basePath: 'https://fastcomments.com'));
final feedPosts = await publicApi.getFeedPostsPublic('YOUR_TENANT_ID');
```

```dart
import 'package:fastcomments_dart/api.dart';

final moderation = ModerationApi(ApiClient(basePath: 'https://fastcomments.com'));
final result = await moderation.getApiComments(
  GetApiCommentsOptions(sso: 'SSO_TOKEN'),
);
```