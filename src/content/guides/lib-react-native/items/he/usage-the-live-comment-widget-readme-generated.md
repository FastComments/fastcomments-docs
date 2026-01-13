ממשק ה-API שונה במעט בהשוואה ל-`fastcomments-react`. ב-native, אנו מעבירים אובייקט config שעוקב אחרי [המבנה הזה](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // מזהה ה-tenant שלך. ניתן לאחזר אותו מ-https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // מזהה או כתובת URL של שרשור התגובות באפליקציה שלך.
  const myAppPageUrl = 'https://example.com/external-page'; // באפשרותך לבחור להגדיר כתובת URL לעמוד חיצוני
  const myAppPageTitle = 'Example Title'; // ... ובגלל זה כנראה תרצה כותרת לתוכן זה
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // על-ידי קריאה ל-setConfig() אנחנו יכולים לבצע פעולות כמו שינוי הדף הנוכחי, או המשתמש המחובר כעת
  // ראה example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```