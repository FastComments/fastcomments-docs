### Главна компонента видџета

Компонента FastCommentsCommentWidget садржи уживо FastComments видџет за коментаре.

Замијените "demo" испод са вашим "tenantId" - доступан [овдје](https://fastcomments.com/auth/my-account/api) у FastComments админ подручју.

Видџет подржава доста опција - погледајте FastCommentsCommentWidgetConfig у src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Ажурирање тренутне странице (за SPA)
Да бисте ажурирали страницу/чланак на који је нит коментара везана, морате ажурирати конфигурационе параметре "urlId" и "url".
Погледајте примјер и објашњење [овдје](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Регион налога (ПАЖЊА: EU корисници)

Ако се налазите у ЕУ, требате обавијестити клијент видџете у којем сте региону. Погледајте [examples/example-eu](/examples/example-eu/src/App.tsx);
У супротном, не морате дефинисати `region`.

### Видџет за број коментара

Компонента FastCommentsCommentCountWidget садржи уживо FastComments видџет за број коментара.

Замијените "demo" испод са вашим "tenantId" - доступан [овдје](https://fastcomments.com/auth/my-account/api) у FastComments админ подручју.

Погледајте FastCommentsCommentCountConfig у src/index.tsx за подржане конфигурационе опције.

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

За потпуно нативну имплементацију FastComments, погледајте [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

За React Native wrapper ове библиотеке, користећи webview, погледајте [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).