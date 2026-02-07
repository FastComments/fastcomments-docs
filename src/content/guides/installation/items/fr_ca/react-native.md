Pour ajouter des commentaires à votre application React Native, vous pouvez trouver notre bibliothèque React Native sur NPM <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">ici</a>.

Le widget de commentaires FastComments pour React Native prend en charge toutes les mêmes fonctionnalités que celui en VanillaJS - commentaires en direct, SSO, etc.

[inline-code-attrs-start title = 'FastComments React Native via NPM'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native via Yarn'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

La configuration est spécifiée légèrement différemment par rapport à la bibliothèque `fastcomments-react` :

[inline-code-attrs-start title = 'Exemple React Native'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Votre identifiant de locataire. Peut être récupéré depuis https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // l'ID ou l'URL du fil de commentaires dans votre application.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Si vous êtes dans l'UE, vous voudrez définir le paramètre `region` :

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

Vous pouvez trouver la configuration prise en charge par le composant React Native <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">ici</a>.