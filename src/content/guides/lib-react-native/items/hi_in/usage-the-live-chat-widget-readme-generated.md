```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // आपकी tenant id। इसे https://fastcomments.com/auth/my-account/api-secret से प्राप्त किया जा सकता है
  const myAppPageId = 'native-test'; // आपके ऐप में कमेंट थ्रेड की ID या URL।
  const myAppPageUrl = 'https://example.com/external-page'; // आप वैकल्पिक रूप से किसी बाहरी पेज का url सेट कर सकते हैं
  const myAppPageTitle = 'Example Title'; // ... और शायद आप इस कंटेंट के लिए एक शीर्षक चाहते हैं
  const config = {
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  };

  return (
      <FastCommentsLiveChatWidget config={config}/>
  );
```