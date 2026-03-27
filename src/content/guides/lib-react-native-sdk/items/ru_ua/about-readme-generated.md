---
Эта библиотека представляет собой полноценную реализацию [FastComments](https://fastcomments.com) для react-native.

Она поддерживает live commenting, chat, threads, emoticons, notifications, SSO, скины и полную настройку путём передачи объекта таблицы стилей. Все ассеты
также можно настраивать, и поддерживается переключение разных ассетов в зависимости от тёмного режима.

Преимущество этой библиотеки в том, что она более гибкая, чем `fastcomments-react-native` wrapper. Комментарии рендерятся с помощью нативных компонентов, а не внутри webview. Примечание: `react-native-webview` по-прежнему требуется как транзитивная зависимость редактора форматированного текста (`@10play/tentap-editor`).

Всё работает на бэкенде FastComments, так что вам нужно только встроить пользовательский интерфейс:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Смотрите [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) для дополнительных примеров.

Добавьте live chat в ваше существующее React Native приложение, или даже создайте социальную сеть!
---