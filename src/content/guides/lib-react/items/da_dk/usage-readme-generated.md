### Hoved-widget-komponenten

The FastCommentsCommentWidget component indeholder den live FastComments kommentar-widget.

Erstat "demo" nedenfor med din "tenantId" - tilgængelig [her](https://fastcomments.com/auth/my-account/api) i FastComments administrationsområdet.

Widget'en understøtter mange muligheder - se FastCommentsCommentWidgetConfig i src/index.tsx.

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

### Kontoregion (VIGTIGT: EU-kunder)

Hvis du er i EU, bør du angive over for klient-widgets, hvilken region du befinder dig i. Se [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
Ellers behøver du ikke definere `region`.

### Kommentaroptællings-widgeten

The FastCommentsCommentCountWidget component indeholder den live FastComments kommentaroptællings-widget.

Erstat "demo" nedenfor med din "tenantId" - tilgængelig [her](https://fastcomments.com/auth/my-account/api) i FastComments administrationsområdet.

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

### Nativ

For en fuldstændig nativ implementering af FastComments, se [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

For en React Native-wrapper af dette bibliotek, der benytter en webview, se [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).