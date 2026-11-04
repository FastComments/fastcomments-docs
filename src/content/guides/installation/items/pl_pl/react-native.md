Aby dodać komentarze do swojej aplikacji React Native, naszą bibliotekę React Native znajdziesz na NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">tutaj</a>.

Widżet komentarzy FastComments React Native obsługuje te same funkcje co wersja VanillaJS - komentarze na żywo, sso i tak dalej.

[inline-code-attrs-start title = 'FastComments React Native przez NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native przez Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Konfiguracja jest określona nieco inaczej niż w bibliotece `fastcomments-react`:

[inline-code-attrs-start title = 'Przykład React Native'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Jeśli znajdujesz się w UE, musisz ustawić parametr `region`:

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

Konfigurację obsługiwaną przez komponent React Native znajdziesz <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">tutaj</a>.