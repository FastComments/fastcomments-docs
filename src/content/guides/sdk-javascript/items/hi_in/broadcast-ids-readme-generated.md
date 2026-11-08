कुछ API कॉल में आपको `broadcastId` पास करना होता है। जब आप इवेंट्स प्राप्त करते हैं, तो आपको यह ID वापस मिल जाएगी, ताकि आप उस इवेंट को अनदेखा कर सकें यदि आप क्लाइंट पर बदलावों को optimistically लागू करने की योजना बना रहे हैं (जो आप शायद करना चाहेंगे क्योंकि यह सबसे बेहतर अनुभव देता है)। यहाँ एक UUID पास करें। ID इतनी unique होनी चाहिए कि वह एक ब्राउज़र सेशन में दो बार न आए।

```typescript
import { v4 as uuidv4 } from 'uuid';

const response = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'page-id',
    comment: 'My comment',
    broadcastId: uuidv4() // इस ऑपरेशन के लिए Unique ID
  }
});
```