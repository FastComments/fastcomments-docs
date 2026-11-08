API `fastcomments-react` से थोड़ा अलग है। Native के साथ, हम एक config ऑब्जेक्ट पास करते हैं जो [इस संरचना](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35) का पालन करता है।

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // आपकी tenant id। इसे https://fastcomments.com/auth/my-account/api-secret से प्राप्त किया जा सकता है
  const myAppPageId = 'native-test'; // आपके ऐप में कमेंट थ्रेड की ID या URL।
  const myAppPageUrl = 'https://example.com/external-page'; // आप वैकल्पिक रूप से किसी बाहरी पेज का URL सेट कर सकते हैं
  const myAppPageTitle = 'Example Title'; // ... और शायद आप इस कंटेंट के लिए एक शीर्षक चाहते हैं
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // setConfig() को कॉल करके, हम वर्तमान पेज या वर्तमान में लॉग इन यूज़र को बदलने जैसी चीज़ें कर सकते हैं
  // example/src/App.tsx देखें

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```