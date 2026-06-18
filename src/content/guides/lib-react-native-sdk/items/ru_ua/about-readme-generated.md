Эта библиотека — полноценная реализация [FastComments](https://fastcomments.com) для react-native.

Она поддерживает живые комментарии, чат, ветки (threads), эмодзи, уведомления, SSO, скины и полную кастомизацию через передачу объекта stylesheet. Все assets
также можно настроить, и поддерживается переключение различных assets в зависимости от dark mode.

Преимущество этой библиотеки в том, что она более гибкая, чем обёртка `fastcomments-react-native`. Комментарии рендерятся с использованием нативных компонентов, а не внутри webview.

Всё работает на бэкенде FastComments, поэтому вам нужно лишь встроить UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Смотрите [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) для дополнительных примеров.

Добавьте живой чат в ваше существующее приложение на React Native или даже создайте социальную сеть!