### Het hoofdwidgetcomponent

De FastCommentsCommentWidget component bevat de live FastComments-commentaarwidget.

Vervang "demo" hieronder door uw "tenantId" - beschikbaar [hier](https://fastcomments.com/auth/my-account/api) in de beheerdersruimte van FastComments.

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

### Het bijwerken van de huidige pagina (voor SPAs)
Om de pagina/artikel waaraan de commentaardraad is gekoppeld bij te werken, moet u de configuratieparameters "urlId" en "url" bijwerken.
Zie het voorbeeld en de uitleg [hier](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Accountregio (LET OP: EU-klanten)

Als u zich in de EU bevindt, wilt u de client-widgets laten weten in welke regio u zich bevindt. Zie [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
Anders hoeft u `region` niet te definiëren.

### De widget voor het aantal reacties

De FastCommentsCommentCountWidget component bevat de live FastComments-widget voor het aantal reacties.

Vervang "demo" hieronder door uw "tenantId" - beschikbaar [hier](https://fastcomments.com/auth/my-account/api) in de beheerdersruimte van FastComments.

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

Voor een React Native-wrapper van deze bibliotheek, die een webview gebruikt, zie [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).