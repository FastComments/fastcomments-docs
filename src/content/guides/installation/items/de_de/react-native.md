Um Kommentare in Ihrer React Native-App hinzuzufügen, finden Sie unsere React Native-Bibliothek auf NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">hier</a>.

Das FastComments React Native Kommentar-Widget unterstützt alle dieselben Funktionen wie das VanillaJS-Widget - Live-Kommentare, SSO und so weiter.

[inline-code-attrs-start title = 'FastComments React Native über NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native über Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Die Konfiguration wird etwas anders angegeben als in der `fastcomments-react`-Bibliothek:

[inline-code-attrs-start title = 'React Native Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Ihre Tenant-ID. Kann von https://fastcomments.com/auth/my-account/api-secret abgerufen werden
  const pageId = 'native-test'; // die ID oder URL des Kommentar-Threads in Ihrer App.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Wenn Sie sich in der EU befinden, sollten Sie den Parameter `region` setzen:

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

Sie finden die Konfiguration, die die React Native-Komponente unterstützt, <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">hier</a>.

---