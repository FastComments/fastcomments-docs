The API немного отличается по сравнению с `fastcomments-react`. В нативной реализации мы передаём объект config, который следует [этой структуре](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Ваш tenant id. Его можно получить с https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID или URL потока комментариев в вашем приложении.
  const myAppPageUrl = 'https://example.com/external-page'; // вы можете опционально задать url внешней страницы
  const myAppPageTitle = 'Example Title'; // ... и, вероятно, вы захотите заголовок для этого контента
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // вызвав setConfig(), мы можем, например, менять текущую страницу или текущего авторизованного пользователя
  // См. example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```