### Ana Widget Bileşeni

FastCommentsCommentWidget bileşeni canlı FastComments yorum widget'ını içerir.

Aşağıdaki "demo"yu FastComments yönetici alanında [burada](https://fastcomments.com/auth/my-account/api) bulunan "tenantId" değerinizle değiştirin.

Widget çok sayıda seçeneği destekler - bkz. FastCommentsCommentWidgetConfig in src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Geçerli Sayfayı Güncelleme (SPA'lar için)
Yorum dizisinin bağlı olduğu sayfa/makale bilgisini güncellemek için yapılandırma parametreleri "urlId" ve "url" değerlerini güncellemelisiniz.
Örneği ve açıklamayı [burada](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx) görün.

### Hesap Bölgesi (DİKKAT: AB Müşterileri)

Eğer AB'deyseniz, istemci widget'larına hangi bölgede olduğunuzu bildirmeniz gerekir. Bakınız [examples/example-eu](/examples/example-eu/src/App.tsx);
Aksi takdirde `region`'ı tanımlamanıza gerek yoktur.

### Yorum Sayacı Bileşeni

FastCommentsCommentCountWidget bileşeni canlı FastComments yorum sayacı widget'ını içerir.

Aşağıdaki "demo"yu FastComments yönetici alanında [burada](https://fastcomments.com/auth/my-account/api) bulunan "tenantId" değerinizle değiştirin.

Desteklenen yapılandırma seçenekleri için FastCommentsCommentCountConfig'e src/index.tsx içinde bakın.

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

FastComments'in tamamen yerel bir uygulaması için bkz. [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Bu kütüphanın bir webview kullanan React Native sarmalayıcısı için bkz. [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).