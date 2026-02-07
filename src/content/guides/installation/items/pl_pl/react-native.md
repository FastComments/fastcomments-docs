Aby dodać komentarze do swojej aplikacji React Native, możesz znaleźć naszą bibliotekę React Native na NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">tutaj</a>.

Widżet komentarzy FastComments dla React Native obsługuje wszystkie te same funkcje co wersja VanillaJS — komentowanie na żywo, SSO i tak dalej.

[inline-code-attrs-start title = 'FastComments React Native przez NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native przez Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Konfiguracja jest określona nieco inaczej w porównaniu z biblioteką `fastcomments-react`:

[inline-code-attrs-start title = 'Przykład React Native'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Twój tenant id. Można pobrać z https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // ID lub URL wątku komentarzy w twojej aplikacji.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Jeśli jesteś w UE, powinieneś ustawić parametr `region`:

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

You can find the configuration the React Native component supports <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">tutaj</a>.

---