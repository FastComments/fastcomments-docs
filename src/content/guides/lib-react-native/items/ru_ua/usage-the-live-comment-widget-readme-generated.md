API немного отличается по сравнению с `fastcomments-react`. В нативной версии мы передаём объект config, который соответствует [этой структуре](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Ваш tenant id. Можно получить с https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID или URL потока комментариев в вашем приложении.
  const myAppPageUrl = 'https://example.com/external-page'; // вы можете опционально задать url внешней страницы
  const myAppPageTitle = 'Example Title'; // ... и, вероятно, вам нужен заголовок для этого контента
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // вызывая setConfig(), мы можем делать такие вещи, как смена текущей страницы или текущего вошедшего пользователя
  // Смотрите example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```
---