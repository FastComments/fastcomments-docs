```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

Помічники SSO включені в пакет (`package:fastcomments_dart/sso/...`).

Клієнт надає три класи API:

- `DefaultApi` - методи, автентифіковані API-ключем, для використання на сервері.
- `PublicApi` - публічні методи, які не потребують API-ключа, безпечні для браузера та мобільних клієнтів.
- `ModerationApi` - обширний набір живих та швидких API модерації. Кожен метод `ModerationApi` приймає параметр `sso` і може автентифікуватися через SSO або cookie сесії FastComments.com.

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