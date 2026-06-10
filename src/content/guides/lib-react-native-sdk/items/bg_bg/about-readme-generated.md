Тази библиотека е пълна react-native имплементация на [FastComments](https://fastcomments.com).

Поддържа живи коментари, чат, нишки, емотикони, известия, SSO, скинове и пълна персонализация чрез подаване на обект stylesheet. Всички ресурси
също могат да бъдат персонализирани и се поддържа превключване на различни ресурси в зависимост от тъмен режим.

Предимството на тази библиотека е, че е по-гъвкава от `fastcomments-react-native` wrapper. Коментарите се рендерират с нативни компоненти, вместо в webview.

Всичко работи на бекенда на FastComments, така че трябва само да внедрите потребителския интерфейс (UI):

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Вижте [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) за повече примери.

Добавете жив чат към съществуващото си приложение на React Native, или дори изградете социална мрежа!