```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO 帮助程序已包含在该包中（`package:fastcomments_dart/sso/...`）。

客户端公开了三个 API 类：

- `DefaultApi` - 用于服务器端的 API 密钥认证方法。
- `PublicApi` - 无需 API 密钥的公共方法，适用于浏览器和移动客户端。
- `ModerationApi` - 一个全面的实时快速审核 API 套件。每个 `ModerationApi` 方法都接受 `sso` 参数，并且可以通过 SSO 或 FastComments.com 会话 Cookie 进行认证。

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