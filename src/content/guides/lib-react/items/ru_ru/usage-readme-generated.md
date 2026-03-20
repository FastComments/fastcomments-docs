### Основной компонент виджета

Компонент FastCommentsCommentWidget содержит виджет комментариев FastComments в режиме реального времени.

Замените "demo" ниже на ваш "tenantId" — он доступен [здесь](https://fastcomments.com/auth/my-account/api) в панели администратора FastComments.

Виджет поддерживает множество опций - см. FastCommentsCommentWidgetConfig в src/index.tsx.

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
См. пример и объяснение [здесь](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Регион аккаунта (ВНИМАНИЕ: клиенты из ЕС)

Если вы находитесь в ЕС, вам следует указать клиентским виджетам, в каком вы регионе. Смотрите [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
В противном случае определять `region` не обязательно.

### Виджет количества комментариев

Компонент FastCommentsCommentCountWidget содержит виджет количества комментариев FastComments в режиме реального времени.

Замените "demo" ниже на ваш "tenantId" — он доступен [здесь](https://fastcomments.com/auth/my-account/api) в панели администратора FastComments.

См. FastCommentsCommentCountConfig в src/index.tsx для поддерживаемых параметров конфигурации.

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