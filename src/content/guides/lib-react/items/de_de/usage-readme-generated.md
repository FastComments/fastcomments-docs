### Die Haupt-Widget-Komponente

Die Komponente FastCommentsCommentWidget enthält das Live-Kommentar-Widget von FastComments.

Ersetze "demo" unten durch deine "tenantId" - verfügbar [hier](https://fastcomments.com/auth/my-account/api) im FastComments-Administrationsbereich.

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

### Aktualisierung der aktuellen Seite (für SPAs)
Um die Seite/den Artikel zu aktualisieren, an den der Kommentar-Thread gebunden ist, musst du die Konfigurationsparameter "urlId" und "url" aktualisieren.
Siehe das Beispiel und die Erklärung [hier](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Konto-Region (ACHTUNG: EU-Kunden)

Wenn du in der EU bist, solltest du den Client-Widgets mitteilen, in welcher Region du dich befindest. Siehe [examples/example-eu](/examples/example-eu/src/App.tsx);
Andernfalls musst du `region` nicht definieren.

### Das Kommentarzähler-Widget

Die Komponente FastCommentsCommentCountWidget enthält das Live-Widget für Kommentaranzahlen von FastComments.

Ersetze "demo" unten durch deine "tenantId" - verfügbar [hier](https://fastcomments.com/auth/my-account/api) im FastComments-Administrationsbereich.

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

Für einen React Native Wrapper dieser Bibliothek, der eine WebView verwendet, siehe [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).