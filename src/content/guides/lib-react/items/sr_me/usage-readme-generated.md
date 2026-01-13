---
### Glavna komponenta видџета

Компонента FastCommentsCommentWidget садржи видџет коментара FastComments у реалном времену.

Замијените "demo" испод са вашим "tenantId" - доступан [овде](https://fastcomments.com/auth/my-account/api) у FastComments администраторском делу.

Видџет подржава много опција - погледајте FastCommentsCommentWidgetConfig у src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Ажурирање тренутне странице (за SPA апликације)
Да бисте ажурирали страницу/чланак за који је везана расправа коментара, морате ажурирати конфигурационе параметре "urlId" и "url".
Погледајте пример и објашњење [овде](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Регион налога (ПАЖЊА: корисници из ЕУ)

Ако сте у ЕУ, требало би да обавестите клијент видџете који регион користите. Погледајте [examples/example-eu](/examples/example-eu/src/App.tsx);
У супротном, не морате дефинисати `region`.

### Видџет броја коментара

Компонента FastCommentsCommentCountWidget садржи видџет броја коментара FastComments у реалном времену.

Замијените "demo" испод са вашим "tenantId" - доступан [овде](https://fastcomments.com/auth/my-account/api) у FastComments администраторском делу.

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

За React Native омот ове библиотеке, који користи webview, погледајте [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).
---