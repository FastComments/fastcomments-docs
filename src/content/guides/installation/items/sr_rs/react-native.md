Да бисте додали коментаре у вашу React Native апликацију, нашу React Native библиотеку на NPM-у можете пронаћи <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">овде</a>.

The FastComments React Native commenting widget supports all of the same features of the VanillaJS one - живо коментарисање, sso, итд.

[inline-code-attrs-start title = 'FastComments React Native преко NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native преко Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Конфигурација се наводи нешто другачије у поређењу са `fastcomments-react` библиотеком:

[inline-code-attrs-start title = 'React Native пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Ваш tenant id. Можете га добити са https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // ID или URL нити коментара у вашој апликацији.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

If you're in the EU, you'll want to set the `region` parameter:

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

You can find the configuration the React Native component supports <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">овде</a>.

---