Чтобы добавить комментарии в ваше приложение React Native, вы можете найти нашу библиотеку React Native на NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">здесь</a>.

Виджет комментирования FastComments для React Native поддерживает все те же функции, что и для VanillaJS — живые комментарии, SSO и т.д.

[inline-code-attrs-start title = 'FastComments React Native через NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native через Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Конфигурация задается немного иначе по сравнению с библиотекой `fastcomments-react`:

[inline-code-attrs-start title = 'Пример React Native'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Ваш tenant id. Его можно получить по адресу https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // ID или URL потока комментариев в вашем приложении.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Если вы находитесь в ЕС, вам следует установить параметр `region`:

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

Вы можете найти конфигурацию, которую поддерживает компонент React Native, <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">здесь</a>.

---