Per aggiungere commenti alla tua app React Native, puoi trovare la nostra libreria React Native su NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">qui</a>.

Il widget di commenti FastComments per React Native supporta tutte le stesse funzionalità di quello VanillaJS - commenti in tempo reale, SSO, ecc.

[inline-code-attrs-start title = 'FastComments React Native tramite NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native tramite Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

La configurazione è specificata leggermente in modo diverso rispetto alla libreria `fastcomments-react`:

[inline-code-attrs-start title = 'Esempio React Native'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Il tuo tenant id. Può essere recuperato da https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // l'ID o l'URL del thread dei commenti nella tua app.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Se ti trovi nell'UE, dovrai impostare il parametro `region`:

[inline-code-attrs-start title = 'React Native - UE'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  // ...
  const config = {
    tenantId: tenantId,
    urlId: pageId,
    region: 'eu'
  };
  // ...
[inline-code-end]

Puoi trovare la configurazione che il componente React Native supporta <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">qui</a>.

---