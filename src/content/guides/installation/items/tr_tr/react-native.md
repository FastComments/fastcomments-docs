React Native uygulamanıza yorum eklemek için React Native kütüphanemizi NPM'de <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">burada</a> bulabilirsiniz.

FastComments React Native yorum widget'ı, VanillaJS olan ile aynı tüm özellikleri destekler - canlı yorum, sso ve benzeri.

[inline-code-attrs-start title = 'FastComments React Native (NPM ile)'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native (Yarn ile)'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Yapılandırma, `fastcomments-react` kütüphanesine kıyasla biraz farklı şekilde belirtilir:

[inline-code-attrs-start title = 'React Native Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Sizin tenant id'niz. Şuradan alınabilir: https://fastcomments.com/auth/my-account/api-secret
  const pageId = 'native-test'; // uygulamanızdaki yorum dizisinin ID'si veya URL'si.
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

Eğer AB'deyseniz, `region` parametresini ayarlamak isteyeceksiniz:

[inline-code-attrs-start title = 'React Native - AB'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
  // ...
  const config = {
    tenantId: tenantId,
    urlId: pageId,
    region: 'eu'
  };
  // ...
[inline-code-end]

React Native bileşeninin desteklediği yapılandırmayı <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts" target="_blank">burada</a> bulabilirsiniz.

---