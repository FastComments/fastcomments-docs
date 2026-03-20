### Główny komponent widgetu

Komponent FastCommentsCommentWidget zawiera działający widget komentarzy FastComments.

Zastąp "demo" poniżej swoim "tenantId" - dostępnym [tutaj](https://fastcomments.com/auth/my-account/api) w panelu administracyjnym FastComments.

Widget obsługuje wiele opcji - zobacz FastCommentsCommentWidgetConfig w src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Aktualizowanie bieżącej strony (dla SPA)

Aby zaktualizować stronę/artykul, do której powiązany jest wątek komentarzy, musisz zaktualizować parametry konfiguracji "urlId" i "url".
Zobacz przykład i wyjaśnienie [tutaj](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Region konta (UWAGA: klienci z UE)

Jeżeli znajdujesz się w UE, powinieneś poinformować widgety klienckie, w jakim regionie się znajdujesz. Zobacz [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
w przeciwnym razie nie musisz definiować `region`.

### Widget licznika komentarzy

Komponent FastCommentsCommentCountWidget zawiera działający widget licznika komentarzy FastComments.

Zastąp "demo" poniżej swoim "tenantId" - dostępnym [tutaj](https://fastcomments.com/auth/my-account/api) w panelu administracyjnym FastComments.

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

Dla całkowicie natywnej implementacji FastComments zobacz [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Dla wrappera tej biblioteki dla React Native, używającego webview, zobacz [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).