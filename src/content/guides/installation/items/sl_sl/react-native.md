Če želite dodati komentarje v vašo React Native aplikacijo, lahko našo knjižnico za React Native najdete na NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">tukaj</a>.

Widget za komentiranje FastComments za React Native podpira vse enake funkcije kot VanillaJS - komentiranje v živo, SSO, in podobno.

[inline-code-attrs-start title = 'FastComments React Native prek NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native prek Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Konfiguracija je določena nekoliko drugače v primerjavi s knjižnico `fastcomments-react`:

[inline-code-attrs-start title = 'Primer React Native'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // ID najemnika. Lahko ga pridobite na https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // ID ali URL niti komentarjev v vaši aplikaciji.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Če se nahajate v EU, boste želeli nastaviti parameter `region`:

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

Konfiguracijo, ki jo komponenta React Native podpira, lahko najdete <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">tukaj</a>.