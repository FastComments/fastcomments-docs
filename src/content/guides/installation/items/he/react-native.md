אתה יכול למצוא את ספריית ה-React Native שלנו ב-NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">כאן</a>.

ווידג'ט התגובות של FastComments ל-React Native תומך בכל אותן התכונות של גרסת VanillaJS - תגובות בזמן אמת, SSO, ועוד.

[inline-code-attrs-start title = 'FastComments React Native דרך NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native דרך Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

ההגדרות מוגדרות בצורה קצת שונה בהשוואה לספריית `fastcomments-react`:

[inline-code-attrs-start title = 'דוגמת React Native'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // ה-tenant id שלך. ניתן לקבל מ-https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // ה-ID או URL של שרשור התגובות באפליקציה שלך.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

אם אתה באיחוד האירופי, תרצה להגדיר את הפרמטר `region`:

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

אתה יכול למצוא את ההגדרות שהקומפוננטה של React Native תומכת בהן <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">כאן</a>.
