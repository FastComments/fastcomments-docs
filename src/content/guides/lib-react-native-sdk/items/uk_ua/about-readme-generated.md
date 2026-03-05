Ця бібліотека — повна реалізація [FastComments](https://fastcomments.com) для react-native.

Вона підтримує коментування в реальному часі, чат, треди, емотикони, сповіщення, SSO, скіни та повну кастомізацію шляхом передачі об'єкта таблиці стилів. Всі ресурси
також можна налаштувати, і підтримується переключення різних ресурсів залежно від темного режиму.

Перевага цієї бібліотеки в тому, що вона більш гнучка і не потребує webview, на відміну від обгортки `fastcomments-react-native`.

Весь функціонал працює на бекенді FastComments, тож вам потрібно лише інтегрувати інтерфейс користувача (UI):

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Дивіться [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) для інших прикладів.

Додайте чат в реальному часі до вашого існуючого додатку React Native, або навіть створіть соціальну мережу!