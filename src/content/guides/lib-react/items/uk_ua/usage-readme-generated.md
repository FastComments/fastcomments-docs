### Основний компонент віджета

Компонент FastCommentsCommentWidget містить живий віджет коментарів FastComments.

Замініть "demo" нижче на ваш "tenantId" — доступний [тут](https://fastcomments.com/auth/my-account/api) в адміністративній частині FastComments.

Віджет підтримує безліч налаштувань — див. FastCommentsCommentWidgetConfig у src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Оновлення поточної сторінки (для SPA-додатків)
Щоб оновити сторінку/статтю, до якої прив'язана нитка коментарів, потрібно оновити параметри конфігурації "urlId" та "url".
Див. приклад та пояснення [тут](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Регіон облікового запису (УВАГА: клієнти з ЄС)

Якщо ви в ЄС, варто вказати клієнтським віджетам ваш регіон. Див. [examples/example-eu](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-eu/src/App.tsx);
Інакше вам не потрібно визначати `region`.

### Віджет лічильника коментарів

Компонент FastCommentsCommentCountWidget містить живий віджет підрахунку коментарів FastComments.

Замініть "demo" нижче на ваш "tenantId" — доступний [тут](https://fastcomments.com/auth/my-account/api) в адміністративній частині FastComments.

Див. FastCommentsCommentCountConfig у src/index.tsx для підтримуваних параметрів конфігурації.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### Нативна реалізація

Для повністю нативної реалізації FastComments див. [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Для оболонки цієї бібліотеки для React Native, що використовує webview, див. [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).