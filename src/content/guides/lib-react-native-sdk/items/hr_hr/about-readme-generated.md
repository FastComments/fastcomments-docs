Ova biblioteka je potpuna react-native implementacija [FastComments](https://fastcomments.com).

Podržava komentiranje uživo, chat, threadove, emotikone, obavijesti, SSO, skinove i potpunu prilagodbu prosljeđivanjem objekta stilova. Svi asseti
također se mogu prilagoditi, a podržano je i prebacivanje različitih asseta ovisno o tamnom načinu rada.

Prednost ove biblioteke je što je fleksibilnija od wrappera `fastcomments-react-native`. Komentari se renderiraju pomoću nativnih komponenti umjesto unutar webview-a. Napomena: `react-native-webview` je i dalje potreban kao tranzitivna ovisnost rich text editora (`@10play/tentap-editor`).

Sve to radi na FastComments backendu, tako da trebate samo integrirati UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Pogledajte [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) za više primjera.

Dodajte chat uživo u svoju postojeću React Native aplikaciju, ili čak izgradite društvenu mrežu!