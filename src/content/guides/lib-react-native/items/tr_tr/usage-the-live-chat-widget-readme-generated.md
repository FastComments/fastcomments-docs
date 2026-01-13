```js
import { FastCommentsLiveChatWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Kiracı kimliğiniz. https://fastcomments.com/auth/my-account/api-secret adresinden alınabilir
  const myAppPageId = 'native-test'; // uygulamanızdaki yorum dizisinin ID'si veya URL'si.
  const myAppPageUrl = 'https://example.com/external-page'; // isteğe bağlı olarak harici bir sayfa için bir url belirtebilirsiniz
  const myAppPageTitle = 'Example Title'; // ... ve muhtemelen bu içerik için bir başlık istersiniz
  const config = {
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  };

  return (
      <FastCommentsLiveChatWidget config={config}/>
  );
```