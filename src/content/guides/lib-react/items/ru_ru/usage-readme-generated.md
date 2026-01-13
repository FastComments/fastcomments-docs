### Основной компонент виджета

Компонент FastCommentsCommentWidget содержит живой виджет комментариев FastComments.

Замените "demo" ниже на ваш "tenantId" - он доступен [здесь](https://fastcomments.com/auth/my-account/api) в админ-панели FastComments.

Виджет поддерживает множество опций - смотрите FastCommentsCommentWidgetConfig в src/index.tsx.

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
Чтобы обновить страницу/статью, к которой привязан поток комментариев, вы должны обновить конфигурационные параметры "urlId" и "url".
См. пример и объяснение [здесь](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Регион аккаунта (ВНИМАНИЕ: клиенты из ЕС)

Если вы находитесь в ЕС, рекомендуется сообщить клиентским виджетам, в каком вы регионе. Смотрите [examples/example-eu](/examples/example-eu/src/App.tsx);
в противном случае вам не нужно определять `region`.

### Виджет счётчика комментариев

Компонент FastCommentsCommentCountWidget содержит живой виджет счётчика комментариев FastComments.

Замените "demo" ниже на ваш "tenantId" - он доступен [здесь](https://fastcomments.com/auth/my-account/api) в админ-панели FastComments.

Смотрите FastCommentsCommentCountConfig в src/index.tsx для поддерживаемых опций конфигурации.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### Нативная реализация

Для полностью нативной реализации FastComments см. [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Для обёртки этой библиотеки для React Native, использующей webview, см. [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).