API се мало разликује у односу на `fastcomments-react`. У native окружењу прослеђујемо config објекат који прати [ову структуру](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Ваш tenant id. Може се преузети са https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID или URL теме коментара у вашој апликацији.
  const myAppPageUrl = 'https://example.com/external-page'; // можете опционално поставити url ка спољној страници
  const myAppPageTitle = 'Example Title'; // ... и вероватно ћете желети наслов за овај садржај
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // позивањем setConfig() можемо урадити ствари као што су промена текуће странице или тренутно пријављеног корисника
  // Погледајте example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```