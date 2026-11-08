```yaml
dependencies:
  fastcomments_dart: ^3.0.0
```

SSO हेल्पर्स पैकेज में शामिल हैं (`package:fastcomments_dart/sso/...`)।

क्लाइंट तीन API क्लासेस उपलब्ध कराता है:

- `DefaultApi` - सर्वर-साइड उपयोग के लिए API-की-प्रमाणीकृत मेथड्स।
- `PublicApi` - सार्वजनिक मेथड्स जिन्हें API की की आवश्यकता नहीं है, ब्राउज़र और मोबाइल क्लाइंट्स के लिए सुरक्षित।
- `ModerationApi` - लाइव और तेज़ मॉडरेशन API का एक विस्तृत सेट। `ModerationApi` का हर मेथड एक `sso` पैरामीटर लेता है और SSO या FastComments.com सेशन कुकी के माध्यम से प्रमाणित हो सकता है।

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