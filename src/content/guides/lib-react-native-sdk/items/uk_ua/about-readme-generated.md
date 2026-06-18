Ця бібліотека є повною реалізацією react-native для [FastComments](https://fastcomments.com).

Вона підтримує живі коментарі, чат, треди, емотикони, сповіщення, SSO, оформлення (skins) та повну кастомізацію шляхом передачі об’єкта stylesheet. Усі ресурси
також можна налаштувати, і підтримується переключення різних ресурсів залежно від режиму темної теми.

Перевага цієї бібліотеки в тому, що вона гнучкіша за обгортку `fastcomments-react-native`. Коментарі рендеряться за допомогою нативних компонентів, а не всередині webview.

Усе працює на бекенді FastComments, тож вам потрібно лише інтегрувати UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Дивіться [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) для додаткових прикладів.

Додайте живий чат до вашого існуючого додатка React Native або навіть створіть соціальну мережу!