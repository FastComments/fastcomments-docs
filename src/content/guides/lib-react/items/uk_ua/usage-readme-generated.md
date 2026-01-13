### Основний компонент віджета

Компонент FastCommentsCommentWidget містить живий віджет коментарів FastComments.

Замініть "demo" нижче на ваш "tenantId" - доступний [тут](https://fastcomments.com/auth/my-account/api) в адмін-панелі FastComments.

Віджет підтримує багато опцій — див. FastCommentsCommentWidgetConfig у src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Оновлення поточної сторінки (для SPA)
Щоб оновити сторінку/статтю, до якої прив’язана тред коментарів, ви повинні оновити параметри конфігурації "urlId" і "url".
Див. приклад та пояснення [тут](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Регіон облікового запису (УВАГА: клієнти з ЄС)

Якщо ви в ЄС, вам потрібно повідомити клієнтські віджети, у якому ви регіоні. Див. [examples/example-eu](/examples/example-eu/src/App.tsx);
Інакше вам не потрібно визначати `region`.

### Віджет лічильника коментарів

Компонент FastCommentsCommentCountWidget містить живий віджет підрахунку коментарів FastComments.

Замініть "demo" нижче на ваш "tenantId" - доступний [тут](https://fastcomments.com/auth/my-account/api) в адмін-панелі FastComments.

Див. FastCommentsCommentCountConfig у src/index.tsx щодо підтримуваних опцій конфігурації.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### Нативна інтеграція

Для повністю нативної реалізації FastComments див. [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

Для обгортки цієї бібліотеки для React Native, що використовує webview, див. [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).