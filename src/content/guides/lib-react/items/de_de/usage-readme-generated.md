### The Main Widget Component

Die Komponente FastCommentsCommentWidget enthält das Live-Kommentar-Widget von FastComments.

Ersetzen Sie "demo" unten durch Ihre "tenantId" - verfügbar [hier](https://fastcomments.com/auth/my-account/api) im FastComments-Adminbereich.

Das Widget unterstützt viele Optionen - siehe FastCommentsCommentWidgetConfig in src/index.tsx.

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
Um die Seite/den Artikel zu aktualisieren, an den der Kommentar-Thread gebunden ist, müssen Sie die Konfigurationsparameter "urlId" und "url" aktualisieren.
Siehe das Beispiel und die Erklärung [hier](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Account Region (ATTENTION: EU Customers)

Wenn Sie in der EU sind, sollten Sie den Client-Widgets mitteilen, in welcher Region Sie sich befinden. Siehe [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
Andernfalls müssen Sie `region` nicht definieren.

### The Comment Count Widget

Die Komponente FastCommentsCommentCountWidget enthält das Live-Widget zur Anzeige der Kommentaranzahl von FastComments.

Ersetzen Sie "demo" unten durch Ihre "tenantId" - verfügbar [hier](https://fastcomments.com/auth/my-account/api) im FastComments-Adminbereich.

Siehe FastCommentsCommentCountConfig in src/index.tsx für die unterstützten Konfigurationsoptionen.

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

Für eine vollständig native Implementierung von FastComments siehe [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Für einen React Native-Wrapper dieser Bibliothek, der eine Webview verwendet, siehe [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).