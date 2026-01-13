### Glavna komponenta pripomočka

Komponenta FastCommentsCommentWidget vsebuje v živo FastComments komentarni pripomoček.

Zamenjajte "demo" spodaj z vašim "tenantId" - na voljo [tukaj](https://fastcomments.com/auth/my-account/api) v skrbniškem območju FastComments.

Pripomoček podpira veliko možnosti - glejte FastCommentsCommentWidgetConfig v src/index.tsx.

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
Če želite posodobiti stran/članek, s katerim je nit komentarjev povezana, morate posodobiti konfiguracijske parametre "urlId" in "url".
Primer in razlago glejte [tukaj](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Regija računa (POZOR: stranke iz EU)

Če ste v EU, boste želeli obvestiti odjemalske widgete, v kateri regiji ste. Glejte [examples/example-eu](/examples/example-eu/src/App.tsx);
sicer vam ni treba definirati `region`.

### Pripomoček za štetje komentarjev

Komponenta FastCommentsCommentCountWidget vsebuje v živo FastComments pripomoček za štetje komentarjev.

Zamenjajte "demo" spodaj z vašim "tenantId" - na voljo [tukaj](https://fastcomments.com/auth/my-account/api) v skrbniškem območju FastComments.

Za podprte konfiguracijske možnosti glejte FastCommentsCommentCountConfig v src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### Nativno

Za popolnoma nativno implementacijo FastComments glejte [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Za React Native ovojnico te knjižnice, ki uporablja webview, glejte [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).