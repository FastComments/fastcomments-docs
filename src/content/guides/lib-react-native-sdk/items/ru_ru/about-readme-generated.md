Эта библиотека является полноценной реализацией react-native [FastComments](https://fastcomments.com).

Она поддерживает live commenting, чат, ветки (threads), эмотиконы, уведомления, SSO, скины и полную кастомизацию путём передачи объекта стилей. Все ресурсы также можно настраивать, и поддерживается переключение разных ресурсов в зависимости от тёмной темы.

Преимущество этой библиотеки в том, что она более гибкая и не требует webview, как обёртка `fastcomments-react-native`.

Всё работает на бэкенде FastComments, поэтому вам нужно только встроить интерфейс (UI):

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

См. [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) для дополнительных примеров.

Добавьте живой чат в ваше существующее приложение на React Native или даже создайте социальную сеть!