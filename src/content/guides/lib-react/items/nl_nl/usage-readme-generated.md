### The Main Widget Component

De FastCommentsCommentWidget-component bevat de live FastComments-reactie-widget.

Vervang "demo" hieronder door je "tenantId" - beschikbaar [hier](https://fastcomments.com/auth/my-account/api) in het FastComments-beheergebied.

De widget ondersteunt veel opties - zie FastCommentsCommentWidgetConfig in src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Updating The Current Page (For SPAs)
Om de pagina/artikel waaraan de reactiedraad is gekoppeld bij te werken, moet je de configuratieparameters "urlId" en "url" bijwerken.
Zie het voorbeeld en de uitleg [hier](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Account Region (ATTENTION: EU Customers)

Als je in de EU bent, moet je aan de client-widgets doorgeven in welke regio je zit. Zie [examples/example-eu](/examples/example-eu/src/App.tsx);
Anders hoef je `region` niet te definiëren.

### The Comment Count Widget

De FastCommentsCommentCountWidget-component bevat de live FastComments-reactietelling-widget.

Vervang "demo" hieronder door je "tenantId" - beschikbaar [hier](https://fastcomments.com/auth/my-account/api) in het FastComments-beheergebied.

Zie FastCommentsCommentCountConfig in src/index.tsx voor de ondersteunde configuratie-opties.

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

Voor een volledig native implementatie van FastComments, zie [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Voor een React Native-wrapper van deze bibliotheek, met gebruik van een webview, zie [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).