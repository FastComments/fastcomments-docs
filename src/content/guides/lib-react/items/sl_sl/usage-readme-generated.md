### Glavna komponenta gradnika

Komponenta FastCommentsCommentWidget vsebuje v živo FastComments komentarni gradnik.

Zamenjajte "demo" spodaj z vašim "tenantId" - na voljo [tukaj](https://fastcomments.com/auth/my-account/api) v administracijskem območju FastComments.

Gradnik podpira veliko možnosti - oglejte si FastCommentsCommentWidgetConfig v src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Posodabljanje trenutne strani (za SPA-je)
Če želite posodobiti stran/članek, na katerega je vezana nit komentarjev, morate posodobiti konfiguracijske parametre "urlId" in "url".
Poglejte primer in razlago [tukaj](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Regija računa (POZOR: stranke iz EU)

Če ste v EU, boste želeli obvestiti odjemalske gradnike, v kateri regiji se nahajate. Oglejte si [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
Sicer vam ni treba definirati `region`.

### Gradnik za štetje komentarjev

Komponenta FastCommentsCommentCountWidget vsebuje v živo FastComments gradnik za štetje komentarjev.

Zamenjajte "demo" spodaj z vašim "tenantId" - na voljo [tukaj](https://fastcomments.com/auth/my-account/api) v administracijskem območju FastComments.

Podprte možnosti konfiguracije si oglejte v FastCommentsCommentCountConfig v src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### Native

Za popolnoma nativno implementacijo FastComments si oglejte [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Za React Native ovojnico te knjižnice, ki uporablja webview, si oglejte [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).