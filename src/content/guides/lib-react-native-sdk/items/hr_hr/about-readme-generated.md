Ova biblioteka je potpuna react-native implementacija [FastComments](https://fastcomments.com).

Podržava live komentiranje, chat, rasprave, emotikone, obavijesti, SSO, skinove, i potpunu prilagodbu prosljeđivanjem objekta s listom stilova. Svi assets
se također mogu prilagoditi, a podržano je prebacivanje različitih assets-a ovisno o tamnom načinu rada.

Prednost ove biblioteke je što je fleksibilnija od `fastcomments-react-native` wrappera. Komentari se renderiraju pomoću nativnih komponenti umjesto unutar webview.

Sve radi na FastComments backendu, tako da trebate samo uključiti UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Pogledajte [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) za više primjera.

Dodajte live chat svojoj postojećoj React Native aplikaciji, ili čak izgradite društvenu mrežu!