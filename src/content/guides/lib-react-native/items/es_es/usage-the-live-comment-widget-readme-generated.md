La API es ligeramente diferente en comparación con `fastcomments-react`. Con la versión nativa, pasamos un objeto config que sigue [esta estructura](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Your tenant id. Can be fetched from https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // the ID or URL of the comment thread in your app.
  const myAppPageUrl = 'https://example.com/external-page'; // you can optional set a url to an external page
  const myAppPageTitle = 'Example Title'; // ... and you probably want a title for this content
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // al llamar a setConfig(), podemos hacer cosas como cambiar la página actual, o el usuario actualmente conectado
  // Consulta example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```
---