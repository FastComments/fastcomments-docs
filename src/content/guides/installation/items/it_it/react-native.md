Puoi trovare la nostra libreria React Native su NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">qui</a>.

Il widget dei commenti FastComments per React Native supporta tutte le stesse funzionalità della versione VanillaJS - commenti in tempo reale, SSO e altro.

[inline-code-attrs-start title = 'FastComments React Native via NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native via Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

La configurazione è specificata in modo leggermente diverso rispetto alla libreria `fastcomments-react`:

[inline-code-attrs-start title = 'React Native Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Your tenant id. Can be fetched from https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // the ID or URL of the comment thread in your app.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Se sei nell'UE, vorrai impostare il parametro `region`:

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

Puoi trovare la configurazione supportata dal componente React Native <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">qui</a>.
