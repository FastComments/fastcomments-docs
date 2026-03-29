Эта библиотека — полная реализация react-native для [FastComments](https://fastcomments.com).

Она поддерживает живые комментарии, чат, треды, эмотиконы, уведомления, SSO, скины и полную настройку путем передачи объекта stylesheet. Все ассеты также можно кастомизировать, и поддерживается переключение разных ассетов в зависимости от тёмной темы.

Преимущество этой библиотеки в том, что она более гибкая, чем обёртка `fastcomments-react-native`. Комментарии рендерятся с помощью нативных компонентов, а не внутри webview. Примечание: `react-native-webview` по-прежнему требуется как транзитивная зависимость редактора богатого текста (`@10play/tentap-editor`).

Всё это работает на бэкенде FastComments, поэтому вам нужно интегрировать только пользовательский интерфейс:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

См. [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) для дополнительных примеров.

Добавьте живой чат в ваше существующее приложение React Native, или даже создайте социальную сеть!