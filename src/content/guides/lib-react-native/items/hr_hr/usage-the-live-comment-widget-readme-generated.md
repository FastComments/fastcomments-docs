API je nešto drugačiji u odnosu na `fastcomments-react`. U native verziji prosljeđujemo konfiguracijski objekt koji slijedi [ovu strukturu](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Vaš tenant id. Može se dohvatiti s https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID ili URL niti komentara u vašoj aplikaciji.
  const myAppPageUrl = 'https://example.com/external-page'; // možete opcionalno postaviti URL na vanjsku stranicu
  const myAppPageTitle = 'Example Title'; // ... i vjerojatno želite naslov za ovaj sadržaj
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // pozivom setConfig() možemo raditi stvari poput promjene trenutne stranice ili trenutno prijavljenog korisnika
  // Pogledajte example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```