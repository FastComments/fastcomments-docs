Našu React Native biblioteku možete pronaći na NPM-u <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">ovdje</a>.

FastComments React Native widget za komentare podržava sve iste značajke kao VanillaJS verzija — komentiranje u stvarnom vremenu, SSO i tako dalje.

[inline-code-attrs-start title = 'FastComments React Native putem NPM-a'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native putem Yarna'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Konfiguracija se specificira malo drugačije u usporedbi s `fastcomments-react` bibliotekom:

[inline-code-attrs-start title = 'React Native primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Vaš tenant id. Može se dohvatiti s https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // ID ili URL niti komentara u vašoj aplikaciji.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Ako ste u EU, željet ćete postaviti parametar `region`:

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
