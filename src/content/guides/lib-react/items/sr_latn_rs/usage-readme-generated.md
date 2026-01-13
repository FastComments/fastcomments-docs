---
### Glavna komponenta widžeta

Komponenta FastCommentsCommentWidget sadrži live FastComments comment widžet.

Zamenite "demo" ispod sa vašim "tenantId" - dostupan [ovde](https://fastcomments.com/auth/my-account/api) u FastComments administratorskom delu.

Widžet podržava mnogo opcija - pogledajte FastCommentsCommentWidgetConfig u src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Ažuriranje trenutne stranice (za SPA aplikacije)
Da biste ažurirali stranicu/artikal za koji je thread komentara vezan, morate ažurirati konfiguracione parametre "urlId" i "url".
Pogledajte primer i objašnjenje [ovde](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Region naloga (PAŽNJA: EU korisnici)

Ako ste u EU, želećete da klijent widžetima kažete u kom regionu se nalazite. Pogledajte [examples/example-eu](/examples/example-eu/src/App.tsx);
Inače, ne morate definisati `region`.

### Widžet za broj komentara

Komponenta FastCommentsCommentCountWidget sadrži live FastComments comment count widžet.

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

Za potpuno nativnu implementaciju FastComments-a, pogledajte [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Za React Native omotač ove biblioteke, koji koristi webview, pogledajte [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).
---