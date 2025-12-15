Можете да намерите нашата React Native библиотека в NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">тук</a>.

FastComments React Native уиджетът за коментари поддържа всички същите функции като VanillaJS версията — коментиране в реално време, SSO и т.н.

[inline-code-attrs-start title = 'FastComments React Native чрез NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native чрез Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Конфигурацията се определя малко по-различно в сравнение с библиотеката `fastcomments-react`:

[inline-code-attrs-start title = 'React Native пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Вашият tenant id. Може да бъде взет от https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // ID или URL на нишката за коментари във вашето приложение
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

Можете да намерите конфигурацията, която React Native компонентът поддържа <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">тук</a>.
