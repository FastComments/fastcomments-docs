```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO 헬퍼는 패키지에 포함되어 있습니다 (`package:fastcomments_dart/sso/...`).

클라이언트는 세 개의 API 클래스를 노출합니다:

- `DefaultApi` - 서버 측 사용을 위한 API 키 인증 메서드.
- `PublicApi` - API 키가 필요 없는 공개 메서드이며, 브라우저 및 모바일 클라이언트에 안전합니다.
- `ModerationApi` - 실시간 및 빠른 모더레이션 API의 포괄적인 스위트입니다. 모든 `ModerationApi` 메서드는 `sso` 매개변수를 받아 SSO 또는 FastComments.com 세션 쿠키를 통해 인증할 수 있습니다.

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