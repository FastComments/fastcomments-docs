A API é ligeiramente diferente em comparação com `fastcomments-react`. Na versão nativa, passamos um objeto de configuração que segue [esta estrutura](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Seu tenant id. Pode ser obtido em https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // O ID ou URL do tópico de comentários no seu app.
  const myAppPageUrl = 'https://example.com/external-page'; // você pode opcionalmente definir uma URL para uma página externa
  const myAppPageTitle = 'Example Title'; // ... e provavelmente você vai querer um título para este conteúdo
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // Ao chamar setConfig(), podemos fazer coisas como alterar a página atual ou o usuário atualmente logado
  // Veja example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```