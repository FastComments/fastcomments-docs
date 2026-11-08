SDK ये API क्लासेस प्रदान करता है:

- **`DefaultApi`** - सुरक्षित एंडपॉइंट्स जिन्हें प्रमाणीकरण के लिए आपकी API key की आवश्यकता होती है। इन्हें सर्वर-साइड ऑपरेशन्स के लिए उपयोग करें।
- **`PublicApi`** - सार्वजनिक एंडपॉइंट्स जिन्हें API key के बिना एक्सेस किया जा सकता है। इन्हें सीधे ब्राउज़र/मोबाइल डिवाइस/आदि से कॉल किया जा सकता है।
- **`ModerationApi`** - मॉडरेटर डैशबोर्ड एंडपॉइंट्स (कमेंट मॉडरेशन, बैन, बैज, ट्रस्ट फैक्टर, सर्च)। मॉडरेटर के सेशन द्वारा प्रमाणित; SSO-प्रमाणित मॉडरेटर्स के लिए `sso` क्वेरी पैरामीटर पास करें।
- **`HiddenApi`** - उन्नत उपयोग के मामलों के लिए आंतरिक/एडमिन एंडपॉइंट्स।

### उदाहरण: Public API का उपयोग (ब्राउज़र-सुरक्षित)

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// Get comments for a page (no API key required)
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### उदाहरण: Default API का उपयोग (केवल सर्वर-साइड)

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // Keep this secret!
});
const defaultApi = new DefaultApi(config);

// Get comments with full admin access
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### उदाहरण: Moderation API का उपयोग

```typescript
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

const sdk = createFastCommentsSDK({ /* basePath, etc. */ });

// Moderator-authenticated calls (session cookie, or pass `sso` for an SSO moderator).
const comments = await sdk.moderationApi.getApiComments({
  tenantId: 'your-tenant-id'
});

await sdk.moderationApi.postSetCommentSpamStatus({
  commentId: 'comment-id',
  spam: true
});
```