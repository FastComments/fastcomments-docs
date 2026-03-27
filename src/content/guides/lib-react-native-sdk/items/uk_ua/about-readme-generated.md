Ця бібліотека — повна реалізація react-native для [FastComments](https://fastcomments.com).

Вона підтримує живі коментарі, чат, треди, емотикони, сповіщення, SSO, скіни та повну кастомізацію шляхом передачі об'єкта stylesheet. Всі ресурси
також можна налаштувати, а також підтримується перемикання різних ресурсів залежно від темного режиму.

Перевага цієї бібліотеки в тому, що вона гнучкіша, ніж обгортка `fastcomments-react-native`. Коментарі відображуються рідними компонентами, а не всередині webview. Примітка: `react-native-webview` все ще потрібен як транзитивна залежність редактора форматованого тексту (`@10play/tentap-editor`).

Усе це працює на бекенді FastComments, тому вам потрібно лише інтегрувати UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Дивіться [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) для додаткових прикладів.

Додайте живий чат у ваш існуючий додаток React Native або навіть створіть соціальну мережу!