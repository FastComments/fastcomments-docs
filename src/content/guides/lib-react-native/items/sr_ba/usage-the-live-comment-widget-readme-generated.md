API је благо другачији у поређењу са `fastcomments-react`. У нативној верзији прослеђујемо конфигурациони објекат који прати [ову структуру](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Ваш tenant id. Може се преузети са https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID или URL нити коментара у вашој апликацији.
  const myAppPageUrl = 'https://example.com/external-page'; // опционално можете поставити URL ка спољној страници
  const myAppPageTitle = 'Example Title'; // ... и вероватно ћете желети наслов за овај садржај
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // позивањем setConfig(), можемо радити ствари попут промене тренутне странице или тренутно пријављеног корисника
  // Погледајте example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```