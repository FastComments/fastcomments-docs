### Główny komponent widżetu

Komponent FastCommentsCommentWidget zawiera widżet komentarzy FastComments działający na żywo.

Zastąp poniżej "demo" swoim "tenantId" - dostępnym [tutaj](https://fastcomments.com/auth/my-account/api) w panelu administracyjnym FastComments.

Widżet obsługuje wiele opcji - zobacz FastCommentsCommentWidgetConfig w src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Aktualizowanie bieżącej strony (dla aplikacji SPA)
Aby zaktualizować stronę/artykul, do której przypisany jest wątek komentarzy, musisz zaktualizować parametry konfiguracji "urlId" i "url".
Zobacz przykład i wyjaśnienie [tutaj](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Region konta (UWAGA: klienci z UE)

Jeśli znajdujesz się w UE, powinieneś poinformować widżety klienckie, w jakim jesteś regionie. Zobacz [examples/example-eu](/examples/example-eu/src/App.tsx);
W przeciwnym razie nie musisz definiować `region`.

### Widżet licznika komentarzy

Komponent FastCommentsCommentCountWidget zawiera działający na żywo widżet licznika komentarzy FastComments.

Zastąp poniżej "demo" swoim "tenantId" - dostępnym [tutaj](https://fastcomments.com/auth/my-account/api) w panelu administracyjnym FastComments.

Zobacz FastCommentsCommentCountConfig w src/index.tsx, aby poznać obsługiwane opcje konfiguracji.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### Natywne

Dla w pełni natywnej implementacji FastComments zobacz [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Dla wrappera tej biblioteki dla React Native, wykorzystującego webview, zobacz [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).