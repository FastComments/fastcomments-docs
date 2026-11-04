```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

פונקציות עזר ל-SSO כלולות בחבילה (`package:fastcomments_dart/sso/...`).

הלקוח חושף שלוש מחלקות API:

- `DefaultApi` - מתודות המאומתות באמצעות מפתח API לשימוש בצד השרת.
- `PublicApi` - מתודות ציבוריות שאינן דורשות מפתח API, בטוחות לשימוש בדפדפן ובלקוחות מובייל.
- `ModerationApi` - סוויטה נרחבת של ממשקי API לניהול (מודרציה) מהיר ובזמן אמת. כל מתודה של `ModerationApi` מקבלת פרמטר `sso` ויכולה לאמת באמצעות SSO או עוגיית סשן של FastComments.com.

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