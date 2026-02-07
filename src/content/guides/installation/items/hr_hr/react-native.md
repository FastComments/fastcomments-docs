Da biste dodali komentare u vašu React Native aplikaciju, možete pronaći našu React Native biblioteku na NPM-u <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">ovdje</a>.

Komentatorski widget FastComments za React Native podržava sve iste značajke kao i VanillaJS verzija — komentiranje uživo, SSO i slično.

[inline-code-attrs-start title = 'FastComments React Native putem NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native putem Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Konfiguracija se navodi nešto drugačije u usporedbi s `fastcomments-react` bibliotekom:

[inline-code-attrs-start title = 'Primjer React Native'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // ID vašeg tenant-a. Može se dohvatiti na https://fastcomments.com/auth/my-account/api-secret
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

Konfiguraciju koju podržava React Native komponenta možete pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ovdje</a>.

---