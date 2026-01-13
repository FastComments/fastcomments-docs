API se nekoliko razlikuje v primerjavi z `fastcomments-react`. Pri native različici posredujemo konfiguracijski objekt, ki sledi [tej strukturi](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Vaš tenant id. Lahko ga pridobite s https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // ID ali URL nitke komentarjev v vaši aplikaciji.
  const myAppPageUrl = 'https://example.com/external-page'; // Neobvezno lahko nastavite URL do zunanje strani
  const myAppPageTitle = 'Example Title'; // ... in verjetno želite naslov za to vsebino
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // Z klicem setConfig() lahko naredimo stvari, kot so sprememba trenutne strani ali trenutno prijavljenega uporabnika
  // Oglejte si example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```