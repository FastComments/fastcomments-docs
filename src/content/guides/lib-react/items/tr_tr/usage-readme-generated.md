### Ana Widget Bileşeni

FastCommentsCommentWidget bileşeni canlı FastComments yorum widget'ını içerir.

Aşağıdaki "demo"yu "tenantId"niz ile değiştirin - FastComments yönetim alanında [burada](https://fastcomments.com/auth/my-account/api) bulunabilir.

Widget birçok seçeneği destekler - desteklenen seçenekler için src/index.tsx içindeki FastCommentsCommentWidgetConfig'e bakın.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Geçerli Sayfanın Güncellenmesi (SPA'lar için)
Yorum dizisinin bağlı olduğu sayfa/makale güncellenecekse yapılandırma parametreleri "urlId" ve "url" güncellenmelidir.
Örnek ve açıklama için [buraya](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx) bakın.

### Hesap Bölgesi (DİKKAT: AB Müşterileri)

Eğer AB'deyseniz, istemci widget'larına hangi bölgede olduğunuzu belirtmek isteyeceksiniz. Örneğe bakın [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
Aksi takdirde `region`'u tanımlamanıza gerek yoktur.

### Yorum Sayacı Widget'ı

FastCommentsCommentCountWidget bileşeni canlı FastComments yorum sayısı widget'ını içerir.

Aşağıdaki "demo"yu "tenantId"niz ile değiştirin - FastComments yönetim alanında [burada](https://fastcomments.com/auth/my-account/api) bulunabilir.

Desteklenen yapılandırma seçenekleri için src/index.tsx içindeki FastCommentsCommentCountConfig'e bakın.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### Yerel

FastComments'ın tamamen yerel bir uygulaması için bkz. [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Bu kütüphanenin bir webview kullanarak React Native sarmalayıcısı için bkz. [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).