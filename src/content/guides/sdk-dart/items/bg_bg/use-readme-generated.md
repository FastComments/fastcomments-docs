```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

Помощните функции за SSO са включени в пакета (`package:fastcomments_dart/sso/...`).

Клиентът предоставя три API класа:

- `DefaultApi` – методи, аутентикирани с API‑ключ, за сървърна употреба.
- `PublicApi` – публични методи, които не изискват API ключ, безопасни за браузъри и мобилни клиенти.
- `ModerationApi` – обширен набор от живи и бързи API‑та за модериране. Всеки метод на `ModerationApi` приема параметър `sso` и може да се аутентицира чрез SSO или с FastComments.com сесийна бисквитка.

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