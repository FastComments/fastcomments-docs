---
Эта библиотека представляет собой полную реализацию react-native для [FastComments](https://fastcomments.com).

Она поддерживает живые комментарии, чат, ветки, эмодзи, уведомления, SSO, скины и полную настройку путем передачи объекта таблицы стилей. Все ресурсы также могут быть настроены, и поддерживается переключение разных ресурсов в зависимости от темного режима.

Преимущество этой библиотеки в том, что она более гибкая, чем обёртка `fastcomments-react-native`. Комментарии рендерятся с помощью нативных компонентов, а не внутри webview.

Все работает на бекенде FastComments, поэтому вам нужно только интегрировать UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Смотрите [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/tree/main/example/src) для дополнительных примеров.

Добавьте живой чат в ваше существующее приложение React Native или даже создайте социальную сеть!
---