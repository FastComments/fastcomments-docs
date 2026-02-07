Para agregar comentarios a tu aplicación React Native, puedes encontrar nuestra biblioteca de React Native en NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">aquí</a>.

El widget de comentarios FastComments para React Native admite todas las mismas características que el de VanillaJS: comentarios en vivo, sso, etc.

[inline-code-attrs-start title = 'FastComments React Native vía NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native vía Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

La configuración se especifica de manera ligeramente diferente en comparación con la biblioteca `fastcomments-react`:

[inline-code-attrs-start title = 'Ejemplo de React Native'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Tu tenant id. Se puede obtener desde https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // El ID o URL del hilo de comentarios en tu aplicación.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Si estás en la UE, querrás establecer el parámetro `region`:

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

Puedes encontrar la configuración que admite el componente de React Native <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">aquí</a>.

---