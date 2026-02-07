Да бисте додали коментаре у вашу React Native апликацију, нашу React Native библиотеку на NPM можете пронаћи <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">овдје</a>.

FastComments React Native видгет за коментаре подржава све исте функције као и VanillaJS - уживо коментарисање, SSO, и тако даље.

[inline-code-attrs-start title = 'FastComments React Native преко NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native преко Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Конфигурација се наводи мало другачије у поређењу са `fastcomments-react` библиотеком:

[inline-code-attrs-start title = 'React Native пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Ваш tenant id. Може се преузети са https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // ID или URL нити коментара у вашој апликацији.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Ако сте у ЕУ, требало би да подесите параметар `region`:

[inline-code-attrs-start title = 'React Native - ЕУ'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  // ...
  const config = {
    tenantId: tenantId,
    urlId: pageId,
    region: 'eu'
  };
  // ...
[inline-code-end]

Конфигурацију коју React Native компонента подржава можете пронаћи <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">овдје</a>.

---