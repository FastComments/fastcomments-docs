Обе `FastCommentsSDK` и `FastCommentsFeedSDK` являются классами `ObservableObject` со свойствами, помеченными `@Published`. Вы можете наблюдать за ними в ваших SwiftUI представлениях для реактивного обновления интерфейса.

### FastCommentsSDK Published Properties

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Общее количество комментариев на сервере |
| `newRootCommentCount` | `Int` | Буферизованные новые комментарии (когда `showLiveRightAway` равно false) |
| `currentUser` | `UserSessionInfo?` | Текущий аутентифицированный пользователь |
| `isSiteAdmin` | `Bool` | Является ли текущий пользователь администратором сайта |
| `isClosed` | `Bool` | Закрыта ли ветка комментариев |
| `hasBillingIssue` | `Bool` | Есть ли проблема с оплатой |
| `isLoading` | `Bool` | Выполняется ли сетевой запрос |
| `hasMore` | `Bool` | Существуют ли дополнительные страницы комментариев |
| `blockingErrorMessage` | `String?` | Ошибка, препятствующая работе интерфейса |
| `warningMessage` | `String?` | Неблокирующее предупреждающее сообщение |
| `isDemo` | `Bool` | Работает ли в демонстрационном режиме |
| `commentsVisible` | `Bool` | Переключатель видимости комментариев |
| `toolbarEnabled` | `Bool` | Показана ли панель форматирования |

### FastCommentsFeedSDK Published Properties

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | В данный момент загруженные посты ленты |
| `hasMore` | `Bool` | Существуют ли дополнительные страницы |
| `currentUser` | `UserSessionInfo?` | Текущий аутентифицированный пользователь |
| `blockingErrorMessage` | `String?` | Блокирующее сообщение об ошибке |
| `isLoading` | `Bool` | Выполняется ли сетевой запрос |
| `newPostsCount` | `Int` | Количество новых постов с последней загрузки |

### Дерево комментариев

К дереву комментариев можно получить доступ через `sdk.commentsTree`:

```swift
// Плоский список видимых узлов для отображения
sdk.commentsTree.visibleNodes

// Поиск комментария по ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---