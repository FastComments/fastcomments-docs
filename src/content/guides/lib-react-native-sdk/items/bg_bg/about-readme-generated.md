Тази библиотека е пълна реализация за react-native на [FastComments](https://fastcomments.com).

Тя поддържа живо коментиране, чат, нишки, емотикони, известия, SSO, скинове и пълна персонализация чрез предаване на обект със стилове. Всички активи
също могат да бъдат персонализирани и се поддържа превключване на различни активи въз основа на тъмен режим.

Предимството на тази библиотека е, че е по-гъвкава и не изисква webview, както обвивката `fastcomments-react-native`.

Всичко работи на бекенда на FastComments, така че трябва само да интегрирате потребителския интерфейс:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Вижте [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) за повече примери.

Добавете жив чат към вашето съществуващо приложение на React Native, или дори изградете социална мрежа!