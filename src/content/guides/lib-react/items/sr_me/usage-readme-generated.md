### Главна компонента виџета

Компонента FastCommentsCommentWidget садржи живи FastComments виџет за коментаре.

Замените "demo" испод са вашим "tenantId" - доступан [овде](https://fastcomments.com/auth/my-account/api) у FastComments административном делу.

Виџет подржава много опција - видите FastCommentsCommentWidgetConfig у src/index.tsx.

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
Погледајте пример и објашњење [овде](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Регион налога (ПОЗОР: корисници из ЕУ)

Ако сте у ЕУ, требало би да обавестите клијент-виџете о региону у којем се налазите. Погледајте [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
У супротном, не морате дефинисати `region`.

### Виџет броја коментара

Компонента FastCommentsCommentCountWidget садржи живи FastComments виџет за број коментара.

Замените "demo" испод са вашим "tenantId" - доступан [овде](https://fastcomments.com/auth/my-account/api) у FastComments административном делу.

Погледајте FastCommentsCommentCountConfig у src/index.tsx за подржане опције конфигурације.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### Нативно

За потпуно нативну имплементацију FastComments, погледајте [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

За React Native омот ове библиотеке, који користи webview, погледајте [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).