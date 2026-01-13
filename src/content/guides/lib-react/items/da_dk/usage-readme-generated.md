### Den primære widget-komponent

Komponenten FastCommentsCommentWidget indeholder den live FastComments-kommentar-widget.

Erstat "demo" nedenfor med dit "tenantId" - tilgængeligt [her](https://fastcomments.com/auth/my-account/api) i FastComments administrationsområdet.

Widgetten understøtter mange indstillinger - se FastCommentsCommentWidgetConfig i src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Opdatering af den aktuelle side (for SPAs)
For at opdatere den side/artikel, som kommentartråden er knyttet til, skal du opdatere konfigurationsparametrene "urlId" og "url".
Se eksemplet og forklaringen [her](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Konto-region (OPMÆRKSOM: EU-kunder)

Hvis du er i EU, bør du fortælle klient-widgets, hvilken region du befinder dig i. Se [examples/example-eu](/examples/example-eu/src/App.tsx); ellers behøver du ikke definere `region`.

### Kommentar-tæller-widgeten

Komponenten FastCommentsCommentCountWidget indeholder den live FastComments-kommentar-tæller-widget.

Erstat "demo" nedenfor med dit "tenantId" - tilgængeligt [her](https://fastcomments.com/auth/my-account/api) i FastComments administrationsområdet.

Se FastCommentsCommentCountConfig i src/index.tsx for de understøttede konfigurationsmuligheder.

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

For en fuldstændig native-implementering af FastComments, se [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

For en React Native-wrapper af dette bibliotek, som bruger en webview, se [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).