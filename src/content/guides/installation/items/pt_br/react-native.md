Para adicionar comentários ao seu aplicativo React Native, você pode encontrar nossa biblioteca React Native no NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">aqui</a>.

O widget de comentários FastComments para React Native oferece todos os mesmos recursos do de VanillaJS - comentários ao vivo, SSO, e assim por diante.

[inline-code-attrs-start title = 'FastComments React Native via NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native via Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

A configuração é especificada de forma um pouco diferente em comparação com a biblioteca `fastcomments-react`:

[inline-code-attrs-start title = 'Exemplo de React Native'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Se você estiver na UE, deverá definir o parâmetro `region`:

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

Você pode encontrar a configuração que o componente React Native suporta <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">aqui</a>.

---