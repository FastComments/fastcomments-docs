### Glavna komponenta widgeta

Komponenta FastCommentsCommentWidget sadrži live FastComments komentar widget.

Zamenite "demo" ispod sa vašim "tenantId" - dostupan [ovde](https://fastcomments.com/auth/my-account/api) u FastComments administratorskom delu.

Widget podržava mnogo opcija - pogledajte FastCommentsCommentWidgetConfig u src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Ažuriranje trenutne stranice (za SPAs)
Da biste ažurirali stranicu/članak kojoj je nit komentara vezana, morate ažurirati parametre konfiguracije "urlId" i "url".
Pogledajte primer i objašnjenje [ovde](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Region naloga (PAŽNJA: EU kupci)

Ako ste u EU, treba da obavestite klijentske widgete u kom regionu se nalazite. Pogledajte [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
Inače, ne morate definiсati `region`.

### Widget za broj komentara

Komponenta FastCommentsCommentCountWidget sadrži live FastComments widget za broj komentara.

Zamenite "demo" ispod sa vašim "tenantId" - dostupan [ovde](https://fastcomments.com/auth/my-account/api) u FastComments administratorskom delu.

Pogledajte FastCommentsCommentCountConfig u src/index.tsx za podržane opcije konfiguracije.

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

Za potpuno nativnu implementaciju FastComments, pogledajte [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Za React Native omotač ove biblioteke koji koristi webview, pogledajte [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).