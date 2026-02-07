Da dodate komentare u svoju React Native aplikaciju, našu React Native biblioteku možete pronaći na NPM-u <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">ovdje</a>.

The FastComments React Native commenting widget supports all of the same features of the VanillaJS one - komentarisanje u stvarnom vremenu, SSO, i slično.

[inline-code-attrs-start title = 'FastComments React Native putem NPM-a'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native putem Yarn-a'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Konfiguracija se malo razlikuje u poređenju sa bibliotekom `fastcomments-react`:

[inline-code-attrs-start title = 'Primjer React Native'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Vaš tenant id. Može se dobiti sa https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // ID ili URL niti komentara u vašoj aplikaciji.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Ako se nalazite u EU, trebali biste postaviti parametar `region`:

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

Konfiguraciju koju React Native komponenta podržava možete pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ovdje</a>.

---