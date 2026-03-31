---
Обе `FastCommentsSDK` и `FastCommentsFeedSDK` являются классами `ObservableObject` с свойствами `@Published`. Вы можете наблюдать за ними в ваших SwiftUI представлениях для реактивных обновлений интерфейса.

### Свойства @Published в FastCommentsSDK

| Свойство | Тип | Описание |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Общее количество комментариев на сервере |
| `newRootCommentCount` | `Int` | Буферизованные новые комментарии (when `showLiveRightAway` is false) |
| `currentUser` | `UserSessionInfo?` | Текущий аутентифицированный пользователь |
| `isSiteAdmin` | `Bool` | Является ли текущий пользователь администратором сайта |
| `isClosed` | `Bool` | Закрыта ли ветка комментариев |
| `hasBillingIssue` | `Bool` | Наличие проблемы с оплатой |
| `isLoading` | `Bool` | Идёт ли сетевой запрос |
| `hasMore` | `Bool` | Существуют ли дополнительные страницы комментариев |
| `blockingErrorMessage` | `String?` | Ошибка, препятствующая функционированию интерфейса |
| `warningMessage` | `String?` | Неблокирующее предупреждение |
| `isDemo` | `Bool` | Запущено ли в демонстрационном режиме |
| `commentsVisible` | `Bool` | Переключатель видимости комментариев |
| `toolbarEnabled` | `Bool` | Отображается ли панель форматирования |

### Свойства @Published в FastCommentsFeedSDK

| Свойство | Тип | Описание |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | В настоящее время загруженные записи ленты |
| `hasMore` | `Bool` | Существуют ли дополнительные страницы |
| `currentUser` | `UserSessionInfo?` | Текущий аутентифицированный пользователь |
| `blockingErrorMessage` | `String?` | Блокирующее сообщение об ошибке |
| `isLoading` | `Bool` | Идёт ли сетевой запрос |
| `newPostsCount` | `Int` | Количество новых записей с момента последней загрузки |

### Дерево комментариев

Дерево комментариев доступно через `sdk.commentsTree`:

```swift
// Плоский список видимых узлов для отрисовки
sdk.commentsTree.visibleNodes

// Поиск комментария по ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---