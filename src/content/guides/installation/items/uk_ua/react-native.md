Щоб додати коментарі до вашого додатка React Native, ви можете знайти нашу бібліотеку React Native на NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">тут</a>.

Компонент коментування FastComments для React Native підтримує всі ті ж функції, що й VanillaJS-версія — живе коментування, SSO тощо.

[inline-code-attrs-start title = 'FastComments React Native через NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native через Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Конфігурація задається трохи інакше порівняно з бібліотекою `fastcomments-react`:

[inline-code-attrs-start title = 'Приклад React Native'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Ваш tenant id. Можна отримати з https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // ID або URL потоку коментарів у вашому додатку.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Якщо ви перебуваєте в ЄС, варто встановити параметр `region`:

[inline-code-attrs-start title = 'React Native - ЄС'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  // ...
  const config = {
    tenantId: tenantId,
    urlId: pageId,
    region: 'eu'
  };
  // ...
[inline-code-end]

Ви можете знайти конфігурацію, яку підтримує компонент React Native, <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">тут</a>.

---