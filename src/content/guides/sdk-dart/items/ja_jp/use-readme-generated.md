```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSOヘルパーはパッケージ（`package:fastcomments_dart/sso/...`）に含まれています。

クライアントは3つのAPIクラスを公開します:

- `DefaultApi` - サーバーサイドでの使用を想定した、APIキーで認証されたメソッド。
- `PublicApi` - APIキーが不要な公開メソッドで、ブラウザやモバイルクライアントでも安全に使用できます。
- `ModerationApi` - 大規模なライブ・高速モデレーションAPIのスイートです。すべての`ModerationApi`メソッドは`sso`パラメータを取り、SSOまたはFastComments.comのセッションクッキーで認証できます。

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