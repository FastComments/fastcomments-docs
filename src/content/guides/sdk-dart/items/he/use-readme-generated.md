```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO helpers are included in the package (`package:fastcomments_dart/sso/...`).

הלקוח חושף שלוש מחלקות API:

- `DefaultApi` - שיטות המאומתות במפתח API לשימוש בצד השרת.
- `PublicApi` - שיטות ציבוריות שאין צורך במפתח API, בטוחות לדפדפן ו
  ללקוחות ניידים.
- `ModerationApi` - חבילה מקיפה של APIיות מודרציה בזמן אמת ומהירות. כל שיטה של `ModerationApi` מקבלת פרמטר `sso` ויכולה לאמת באמצעות SSO או באמצעות עוגיית סשן של FastComments.com.

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