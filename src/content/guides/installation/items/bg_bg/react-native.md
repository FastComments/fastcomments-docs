За да добавите коментари към вашето React Native приложение, можете да намерите нашата библиотека за React Native в NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">тук</a>.

Компонентът за коментари FastComments за React Native поддържа всички същите функции като този за VanillaJS - коментари в реално време, sso и т.н.

[inline-code-attrs-start title = 'FastComments React Native чрез NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native чрез Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Конфигурацията се задава малко по-различно в сравнение с библиотеката `fastcomments-react`:

[inline-code-attrs-start title = 'Пример за React Native'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Вашият tenant id. Може да бъде получен от https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // ID или URL на нишката с коментари във вашето приложение.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Ако сте в ЕС, ще искате да зададете параметъра `region`:

[inline-code-attrs-start title = 'React Native - ЕС'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  // ...
  const config = {
    tenantId: tenantId,
    urlId: pageId,
    region: 'eu'
  };
  // ...
[inline-code-end]

Можете да намерите конфигурацията, която компонентът за React Native поддържа <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">тук</a>.

---