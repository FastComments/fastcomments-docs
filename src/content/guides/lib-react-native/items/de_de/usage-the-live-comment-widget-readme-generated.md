---
Die API unterscheidet sich leicht von `fastcomments-react`. Bei der nativen Version übergeben wir ein Konfigurationsobjekt, das [dieser Struktur](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35) folgt.

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Ihre Tenant-ID. Kann von https://fastcomments.com/auth/my-account/api-secret abgerufen werden
  const myAppPageId = 'native-test'; // Die ID oder URL des Kommentar-Threads in Ihrer App.
  const myAppPageUrl = 'https://example.com/external-page'; // optional können Sie eine URL zu einer externen Seite angeben
  const myAppPageTitle = 'Example Title'; // ... und Sie möchten wahrscheinlich einen Titel für diesen Inhalt
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // Durch Aufrufen von setConfig() können wir z. B. die aktuelle Seite oder den aktuell angemeldeten Benutzer ändern
  // Siehe example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```
---