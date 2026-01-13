API је благо другачији у односу на `fastcomments-react`. У native верзији прослеђујемо конфигурациони објекат који прати [ову структуру](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Ваш tenant id. Може се добити са https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID или URL нити коментара у вашој апликацији.
  const myAppPageUrl = 'https://example.com/external-page'; // можете по избор поставити URL ка екстерној страници
  const myAppPageTitle = 'Example Title'; // ... и вероватно ћете желети наслов за овај садржај
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // позивањем setConfig() можемо радити ствари као што су промена тренутне странице или тренутно пријављеног корисника
  // Погледајте example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```