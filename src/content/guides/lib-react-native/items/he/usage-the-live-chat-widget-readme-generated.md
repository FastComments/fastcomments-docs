```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // מזהה השוכר שלך. ניתן להשיג אותו מ https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ה-ID או ה-URL של שרשור התגובות באפליקציה שלך.
  const myAppPageUrl = 'https://example.com/external-page'; // ניתן באופן אופציונלי להגדיר URL לדף חיצוני
  const myAppPageTitle = 'Example Title'; // ... ואולי תרצה כותרת לתוכן זה
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