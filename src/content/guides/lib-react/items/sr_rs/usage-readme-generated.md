### Главна компонента видгета

Компонента FastCommentsCommentWidget садржи уживо FastComments коментарски видгет.

Замените "demo" испод са вашим "tenantId" - доступан [here](https://fastcomments.com/auth/my-account/api) у FastComments администраторској зони.

Видгет подржава много опција - видите FastCommentsCommentWidgetConfig у src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Ажурирање тренутне странице (за SPA-ове)
Да бисте ажурирали страницу/чланак са којим је нит коментара повезана, морате ажурирати конфигурационе параметре "urlId" и "url".
Погледајте пример и објашњење [here](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Регион налога (ПАЖЊА: корисници из ЕУ)

Ако сте у ЕУ, требало би да обавестите клијент видгете у којем сте региону. Погледајте [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
У супротном, не морате дефинисати `region`.

### Видгет броја коментара

Компонента FastCommentsCommentCountWidget садржи уживо FastComments видгет броја коментара.

Замените "demo" испод са вашим "tenantId" - доступан [here](https://fastcomments.com/auth/my-account/api) у FastComments администраторској зони.

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

За потпуно нативну имплементацију FastComments-а, погледајте [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

За React Native омотач ове библиотеке, који користи webview, погледајте [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).