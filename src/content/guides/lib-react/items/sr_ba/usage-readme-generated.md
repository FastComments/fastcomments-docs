### Glavna komponenta widgeta

Komponenta FastCommentsCommentWidget sadrži uživo FastComments komentar widget.

Zamijenite "demo" ispod sa vašim "tenantId" - dostupno [ovdje](https://fastcomments.com/auth/my-account/api) u FastComments administratorskom području.

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

### Ažuriranje trenutne stranice (za SPA-ove)
Da biste ažurirali stranicu/članak na koji je komentar nit vezana, morate ažurirati konfiguracione parametre "urlId" i "url".
Pogledajte primjer i objašnjenje [ovdje](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Account Region (PAŽNJA: EU korisnici)

Ako ste u EU, trebate klijentskim widgetima reći u kojoj regiji se nalazite. Pogledajte [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
U suprotnom, ne morate definirati `region`.

### Widget za broj komentara

Komponenta FastCommentsCommentCountWidget sadrži uživo FastComments widget za broj komentara.

Zamijenite "demo" ispod sa vašim "tenantId" - dostupno [ovdje](https://fastcomments.com/auth/my-account/api) u FastComments administratorskom području.

Pogledajte FastCommentsCommentCountConfig u src/index.tsx za podržane konfiguracione opcije.

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

Za potpuno nativnu implementaciju FastComments, pogledajte [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Za React Native omotač ove biblioteke, koji koristi webview, pogledajte [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).