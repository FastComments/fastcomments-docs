Naszą bibliotekę React Native możesz znaleźć na NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">tutaj</a>.

Widget komentarzy FastComments dla React Native obsługuje wszystkie te same funkcje co wersja VanillaJS - komentarze na żywo, SSO i więcej.

[inline-code-attrs-start title = 'FastComments React Native via NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native via Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Konfiguracja jest określana nieco inaczej w porównaniu z biblioteką `fastcomments-react`:

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

Jeśli jesteś w UE, będziesz chciał ustawić parametr `region`:

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

Konfigurację obsługiwaną przez komponent React Native możesz znaleźć <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">tutaj</a>.
