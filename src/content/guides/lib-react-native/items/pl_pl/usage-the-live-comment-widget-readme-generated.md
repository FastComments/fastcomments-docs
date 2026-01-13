The API jest nieco inne w porównaniu do `fastcomments-react`. W wersji natywnej przekazujemy obiekt konfiguracyjny, który ma [tę strukturę](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Twój tenant id. Można go pobrać z https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID lub URL wątku komentarzy w Twojej aplikacji.
  const myAppPageUrl = 'https://example.com/external-page'; // opcjonalnie możesz ustawić URL prowadzący do zewnętrznej strony
  const myAppPageTitle = 'Example Title'; // ... i prawdopodobnie chcesz tytuł dla tej treści
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // wywołując setConfig(), możemy robić rzeczy takie jak zmiana aktualnej strony lub aktualnie zalogowanego użytkownika
  // Zobacz example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```