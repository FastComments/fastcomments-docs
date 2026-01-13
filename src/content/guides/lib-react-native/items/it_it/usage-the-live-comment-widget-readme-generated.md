L'API è leggermente diversa rispetto a `fastcomments-react`. Con la versione nativa, passiamo un oggetto config che segue [questa struttura](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Il tuo tenant id. Può essere recuperato da https://fastcomments.com/auth/my-account/api-secret
  const myAppPageId = 'native-test'; // L'ID o l'URL del thread di commenti nella tua app.
  const myAppPageUrl = 'https://example.com/external-page'; // puoi opzionalmente impostare un URL per una pagina esterna
  const myAppPageTitle = 'Example Title'; // ... e probabilmente vorrai un titolo per questo contenuto
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // chiamando setConfig(), possiamo fare cose come cambiare la pagina corrente o l'utente attualmente connesso
  // Vedi example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```