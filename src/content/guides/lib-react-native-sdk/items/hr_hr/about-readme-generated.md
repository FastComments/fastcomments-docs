Ova biblioteka predstavlja kompletnu react-native implementaciju [FastComments](https://fastcomments.com).

Podržava komentiranje uživo, chat, threadove, emotikone, obavijesti, SSO, skins i potpunu prilagodbu prosljeđivanjem objekta stylesheet. Svi resursi
također se mogu prilagoditi, a podržano je prebacivanje različitih resursa ovisno o tamnom načinu rada.

Prednost ove biblioteke je što je fleksibilnija i ne zahtijeva webview, poput `fastcomments-react-native` wrappera.

Sve to radi na FastComments backendu, pa morate samo integrirati korisničko sučelje:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Pogledajte [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) za više primjera.

Dodajte chat uživo u svoju postojeću React Native aplikaciju, ili čak izgradite društvenu mrežu!