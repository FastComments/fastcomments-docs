### Главна компонента видгета

Компонента FastCommentsCommentWidget садржи уживо FastComments коментарски видгет.

Замените "demo" испод са вашим "tenantId" - доступан [овде](https://fastcomments.com/auth/my-account/api) у админ делу FastComments.

Видгет подржава много опција - погледајте FastCommentsCommentWidgetConfig у src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Ажурирање тренутне странице (за једностраничне апликације)
Да бисте ажурирали страницу/чланак са којим је нит коментара повезана, морате ажурирати конфигурационе параметре "urlId" и "url".
Погледајте пример и објашњење [овде](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Регија налога (ПАЖЊА: корисници из ЕУ)

Ако сте у ЕУ, требало би да клијентским видгетима назначите у којој се регији налазите. Погледајте [examples/example-eu](/examples/example-eu/src/App.tsx);
У супротном, не морате дефинисати `region`.

### Видгет броја коментара

Компонента FastCommentsCommentCountWidget садржи уживо FastComments видгет броја коментара.

Замените "demo" испод са вашим "tenantId" - доступан [овде](https://fastcomments.com/auth/my-account/api) у админ делу FastComments.

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

За React Native wrapper ове библиотеке, који користи webview, погледајте [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).