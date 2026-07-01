```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO helpers are included in the package (`package:fastcomments_dart/sso/...`).

The client exposes three API classes:

- `DefaultApi` - API-key-authenticated methods for server-side use.
- `PublicApi` - public methods that need no API key, safe for browser and
  mobile clients.
- `ModerationApi` - an extensive suite of live and fast moderation APIs. Every `ModerationApi` method takes an `sso` parameter and can authenticate via SSO or a FastComments.com session cookie.

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