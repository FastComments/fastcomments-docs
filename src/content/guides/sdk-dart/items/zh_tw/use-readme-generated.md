```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO 輔助工具已包含在套件中（`package:fastcomments_dart/sso/...`）。

客戶端提供三個 API 類別：

- `DefaultApi` - 透過 API 金鑰驗證的伺服器端使用方法。
- `PublicApi` - 無需 API 金鑰的公開方法，適用於瀏覽器和行動客戶端。
- `ModerationApi` - 包含大量即時且快速的審核 API。每個 `ModerationApi` 方法都接受 `sso` 參數，並可透過 SSO 或 FastComments.com 的會話 Cookie 進行驗證。

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