### Die Haupt-Widget-Komponente

Die FastCommentsCommentWidget-Komponente enthält das Live-FastComments-Kommentar-Widget.

Ersetzen Sie unten "demo" durch Ihre "tenantId" – verfügbar [hier](https://fastcomments.com/auth/my-account/api) im FastComments-Adminbereich.

Das Widget unterstützt viele Optionen – siehe FastCommentsCommentWidgetConfig in src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Aktualisieren der aktuellen Seite (für SPAs)
Um die Seite/den Artikel zu aktualisieren, mit dem der Kommentar-Thread verknüpft ist, müssen Sie die Konfigurationsparameter "urlId" und "url" aktualisieren.
Sehen Sie sich das Beispiel und die Erklärung [hier](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx) an.

### Konto-Region (ACHTUNG: EU-Kunden)

Wenn Sie in der EU sind, müssen Sie den Client-Widgets mitteilen, in welcher Region Sie sich befinden. Siehe [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
Andernfalls müssen Sie `region` nicht definieren.

### Das Kommentaranzahl-Widget

Die FastCommentsCommentCountWidget-Komponente enthält das Live-FastComments-Kommentaranzahl-Widget.

Ersetzen Sie unten "demo" durch Ihre "tenantId" – verfügbar [hier](https://fastcomments.com/auth/my-account/api) im FastComments-Adminbereich.

Die unterstützten Konfigurationsoptionen finden Sie in FastCommentsCommentCountConfig in src/index.tsx.

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

Eine vollständig native Implementierung von FastComments finden Sie unter [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Einen React-Native-Wrapper dieser Bibliothek, der ein Webview verwendet, finden Sie unter [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).