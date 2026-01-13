The API je neznatno drugačiji u odnosu na `fastcomments-react`. U native verziji prosleđujemo config objekat koji odgovara [ovoj strukturi](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Vaš tenant id. Može se preuzeti sa https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID ili URL niti komentara u vašoj aplikaciji.
  const myAppPageUrl = 'https://example.com/external-page'; // možete opcionalno postaviti URL ka eksternoj stranici
  const myAppPageTitle = 'Example Title'; // ... i verovatno želite naslov za ovaj sadržaj
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // pozivanjem setConfig(), možemo raditi stvari kao što su promena trenutne stranice ili trenutno prijavljenog korisnika
  // Pogledajte example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```