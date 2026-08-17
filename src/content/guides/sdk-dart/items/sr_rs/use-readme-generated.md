```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO помоћници су укључени у пакет (`package:fastcomments_dart/sso/...`).

Клијент излаже три API класе:

- `DefaultApi` - методе аутентификоване API кључем за серверско коришћење.
- `PublicApi` - јавне методе које не захтевају API кључ, безбедне за прегледаче и  
  мобилне клијенте.
- `ModerationApi` - обиман скуп живих и брзих API‑ја за модерацију. Свака `ModerationApi` метода прихвата `sso` параметар и може се аутентификоваћe путем SSO‑а или FastComments.com сесијског колачића.

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