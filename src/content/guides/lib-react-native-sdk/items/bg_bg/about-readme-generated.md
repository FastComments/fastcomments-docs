---
Тази библиотека е пълна реализация за react-native на [FastComments](https://fastcomments.com).

Поддържа коментиране в реално време, чат, нишки, емотикони, известия, SSO, скинове и пълна персонализация чрез подаване на обект stylesheet. Всички assets също могат да бъдат персонализирани, и се поддържа превключване на различни assets в зависимост от тъмния режим.

Предимството на тази библиотека е, че е по-гъвкава от обвивката `fastcomments-react-native`. Коментарите се рендират с native компоненти, а не вътре в webview. Забележка: `react-native-webview` все още е необходим като транзитивна зависимост на rich text редактора (`@10play/tentap-editor`).

Всичко това работи върху бекенда на FastComments, така че трябва само да интегрирате UI-то:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Вижте [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) за повече примери.

Добавете live чат към съществуващото си React Native приложение, или дори изградете социална мрежа!
---