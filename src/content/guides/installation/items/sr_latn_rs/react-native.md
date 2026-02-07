Da biste dodali komentare u vašu React Native aplikaciju, možete pronaći našu React Native biblioteku na NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">ovde</a>.

FastComments React Native vidžet za komentare podržava sve iste funkcionalnosti kao i VanillaJS verzija - komentarisanje uživo, SSO i tako dalje.

[inline-code-attrs-start title = 'FastComments React Native putem NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native putem Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Konfiguracija se specificira malo drugačije u odnosu na biblioteku `fastcomments-react`:

[inline-code-attrs-start title = 'Primer za React Native'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Vaš tenant id. Može se preuzeti sa https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // ID ili URL niti komentara u vašoj aplikaciji.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Ako se nalazite u EU, trebalo bi da postavite `region` parametar:

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

---