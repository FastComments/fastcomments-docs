Om reacties aan je React Native-app toe te voegen, kun je onze React Native-bibliotheek op NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">hier</a> vinden.

De FastComments React Native-commentaar-widget ondersteunt alle dezelfde functies als die van de VanillaJS-versie - realtime reacties, sso, en zo verder.

[inline-code-attrs-start title = 'FastComments React Native via NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native via Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

De configuratie wordt iets anders opgegeven vergeleken met de `fastcomments-react`-bibliotheek:

[inline-code-attrs-start title = 'React Native Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Je tenant-id. Kan worden opgehaald van https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // De ID of URL van de discussiedraad in je app.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Als je in de EU bent, wil je de `region`-parameter instellen:

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

Je kunt de configuratie die de React Native-component ondersteunt <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">hier</a> vinden.

---