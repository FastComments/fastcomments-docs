### Основният компонент на уиджета

Компонентът FastCommentsCommentWidget съдържа джаджа за коментари на FastComments в реално време.

Заменете "demo" по-долу с вашия "tenantId" - наличен [тук](https://fastcomments.com/auth/my-account/api) в администраторския панел на FastComments.

Джаджата поддържа много опции - вижте FastCommentsCommentWidgetConfig в src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Актуализиране на текущата страница (за SPA приложения)
За да актуализирате страницата/статията, към която е привързана нишката с коментари, трябва да актуализирате конфигурационните параметри "urlId" и "url".
Вижте примера и обяснението [тук](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Регион на акаунта (ВНИМАНИЕ: клиенти от ЕС)

Ако сте в ЕС, ще искате да уведомите клиентските джаджи в кой регион се намирате. Вижте [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
В противен случай не е необходимо да дефинирате `region`.

### Джаджата за брой коментари

Компонентът FastCommentsCommentCountWidget съдържа джаджа за брой коментари на FastComments в реално време.

Заменете "demo" по-долу с вашия "tenantId" - наличен [тук](https://fastcomments.com/auth/my-account/api) в администраторския панел на FastComments.

Вижте FastCommentsCommentCountConfig в src/index.tsx за поддържаните опции за конфигурация.

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

За напълно нативна имплементация на FastComments вижте [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

За React Native обвивка на тази библиотека, използваща webview, вижте [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).