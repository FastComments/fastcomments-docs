यदि आपने FastComments को Vercel Marketplace इंटीग्रेशन के माध्यम से इंस्टॉल किया है, तो आपकी tenant ID `FASTCOMMENTS_TENANT_ID` environment variable के रूप में उपलब्ध है। इसे क्लाइंट पर पढ़ने के लिए, इसे `next.config.js` के माध्यम से expose करें या इसके आगे `NEXT_PUBLIC_` जोड़ें:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```