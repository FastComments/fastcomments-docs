React Native kütüphanemizi NPM'de <a href="https://www.npmjs.com/package/fastcomments-react-native" target="_blank">burada</a> bulabilirsiniz.

FastComments React Native yorum widget'ı, VanillaJS olanla aynı tüm özellikleri destekler - canlı yorumlama, SSO vb.

[inline-code-attrs-start title = 'FastComments React Native (NPM ile)'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install --save fastcomments-react-native
[inline-code-end]


[inline-code-attrs-start title = 'FastComments React Native (Yarn ile)'; type = 'shell'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
yarn add fastcomments-react-native
[inline-code-end]

Yapılandırma, `fastcomments-react` kütüphanesine kıyasla biraz farklı belirtilir:

[inline-code-attrs-start title = 'React Native Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import { FastCommentsCommentWidget } from 'fastcomments-react-native';

// ...

  const tenantId = 'demo'; // Kiracı kimliğiniz. https://fastcomments.com/auth/my-account/api-secret adresinden alınabilir
  const pageId = 'native-test'; // Uygulamanızdaki yorum dizisinin ID'si veya URL'si
  const config = {
    tenantId: tenantId,
    urlId: pageId
  };

  return (
      <FastCommentsCommentWidget config={config}/>
  );
[inline-code-end]

AB'deyseniz, `region` parametresini ayarlamanız gerekir:

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
