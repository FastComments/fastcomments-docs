### Основной компонент виджета

Компонент FastCommentsCommentWidget содержит живой виджет комментариев FastComments.

Замените "demo" ниже на ваш "tenantId" — он доступен [здесь](https://fastcomments.com/auth/my-account/api) в административной панели FastComments.

Виджет поддерживает множество параметров — см. FastCommentsCommentWidgetConfig в src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Обновление текущей страницы (для SPA)
Чтобы обновить страницу/статью, к которой привязан поток комментариев, вы должны обновить параметры конфигурации "urlId" и "url".
Смотрите пример и объяснение [здесь](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Регион аккаунта (ВНИМАНИЕ: клиенты из ЕС)

Если вы в ЕС, вам нужно указать виджетам клиента, в каком вы регионе. Смотрите [examples/example-eu](/examples/example-eu/src/App.tsx);
В противном случае определять `region` не обязательно.

### Виджет количества комментариев

Компонент FastCommentsCommentCountWidget содержит живой виджет подсчёта комментариев FastComments.

Замените "demo" ниже на ваш "tenantId" — он доступен [здесь](https://fastcomments.com/auth/my-account/api) в административной панели FastComments.

Смотрите FastCommentsCommentCountConfig в src/index.tsx для поддерживаемых параметров конфигурации.

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

Для полностью нативной реализации FastComments смотрите [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Для обёртки этой библиотеки для React Native, использующей webview, смотрите [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).