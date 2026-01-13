Эта библиотека — полноценная реализация react-native [FastComments](https://fastcomments.com).

Она поддерживает живые комментарии, чат, треды, смайлики, уведомления, SSO, скины и полную настройку через передачу объекта таблицы стилей. Все ресурсы
также могут быть настроены, и поддерживается переключение различных ресурсов в зависимости от режима «тёмная тема».

Преимущество этой библиотеки в том, что она более гибкая и не требует webview, в отличие от обёртки `fastcomments-react-native`.

Всё работает на бэкенде FastComments, поэтому вам нужно только встроить UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Смотрите [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) для дополнительных примеров.

Добавьте живой чат в ваше существующее приложение на React Native или даже создайте социальную сеть!