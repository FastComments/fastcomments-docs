API-то е леко различно в сравнение с `fastcomments-react`. При нативна интеграция предаваме обект config, който следва [тази структура](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Вашият tenant id. Може да бъде получен от https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID или URL на нишката за коментари във вашето приложение.
  const myAppPageUrl = 'https://example.com/external-page'; // можете по избор да зададете URL към външна страница
  const myAppPageTitle = 'Example Title'; // ... и вероятно ще искате заглавие за това съдържание
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // чрез извикване на setConfig() можем да правим неща като смяна на текущата страница или на потребителя, който е влязъл
  // Вижте example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```