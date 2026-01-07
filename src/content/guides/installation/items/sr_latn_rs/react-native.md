Našu React Native biblioteku možete pronaći na NPM-u <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">ovde</a>.

FastComments React Native vidžet za komentare podržava sve iste funkcije kao VanillaJS verzija — komentarisanje u realnom vremenu, SSO i tako dalje.

[inline-code-attrs-start title = 'FastComments React Native preko NPM-a'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native preko Yarn-a'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Konfiguracija se specificira malo drugačije u poređenju sa `fastcomments-react` bibliotekom:

[inline-code-attrs-start title = 'React Native primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Ako ste u EU, želećete da podesite parametar `region`:

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

Konfiguraciju koju React Native komponenta podržava možete pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ovde</a>.
