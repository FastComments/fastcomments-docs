כדי להוסיף הערות לאפליקציית React Native שלך, ניתן למצוא את ספריית React Native שלנו ב-NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">כאן</a>.

ווידג'ט ההערות של FastComments ל-React Native תומך בכל אותן התכונות כמו של VanillaJS - live commenting, sso, וכן הלאה.

[inline-code-attrs-start title = 'FastComments React Native באמצעות NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native באמצעות Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

הקונפיגורציה מוגדרת מעט שונה בהשוואה לספריית `fastcomments-react`:

[inline-code-attrs-start title = 'דוגמה ל-React Native'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // מזהה tenant שלך. ניתן לקבלו מ-https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // ה-ID או ה-URL של אשכול התגובות באפליקציה שלך.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

אם אתם באירופה, כדאי להגדיר את הפרמטר `region`:

[inline-code-attrs-start title = 'React Native - EU'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  // ...
  const config = {
    tenantId: tenantId,
    urlId: pageId,
    region: 'eu'
  };
  // ...
[inline-code-end]

ניתן למצוא את התצורה שהקומפוננטה של React Native תומכת בה <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">כאן</a>.

---