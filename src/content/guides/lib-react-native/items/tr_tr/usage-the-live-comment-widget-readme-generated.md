The API, `fastcomments-react` ile karşılaştırıldığında biraz farklıdır. Native'de, şu yapıyı izleyen bir config nesnesi geçiririz: [bu yapı](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L35).

```js
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const myTenantId = 'demo'; // Kiracı kimliğiniz. https://fastcomments.com/auth/my-account/api-secret adresinden alınabilir
  const myAppPageId = 'native-test'; // uygulamanızdaki yorum dizisinin ID'si veya URL'si.
  const myAppPageUrl = 'https://example.com/external-page'; // isteğe bağlı olarak harici bir sayfaya URL ayarlayabilirsiniz
  const myAppPageTitle = 'Example Title'; // ... ve muhtemelen bu içerik için bir başlık istersiniz
  const [config, setConfig] = useState({
    tenantId: myTenantId,
    urlId: myAppPageId,
    url: myAppPageUrl,
    pageTitle: myAppPageTitle
  });

  // setConfig() çağrısı ile mevcut sayfayı veya şu anda giriş yapmış kullanıcıyı değiştirmek gibi işlemler yapabiliriz
  // Örnek için bkz. example/src/App.tsx

  return (
      <FastCommentsCommentWidget config={config}/>
  );
```