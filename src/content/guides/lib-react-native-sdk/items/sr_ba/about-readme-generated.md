---
Ova biblioteka je potpuna react-native implementacija FastComments-a.

Podržava live komentarisanje, chat, thread-ove, emotikone, notifikacije, SSO, skinove i potpunu prilagodbu prosljeđivanjem stylesheet object-a. Svi assets se takođe mogu prilagoditi, i podržava prebacivanje različitih assets-a bazirano na dark mode-u.

Prednost ove biblioteke je što je fleksibilnija od `fastcomments-react-native` wrapper-a. Komentari se renderuju sa native komponentama umjesto unutar webview.

Sve radi na FastComments backend-u, tako da trebate integrisati samo UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Pogledajte [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) za više primjera.

Dodajte live chat svojoj postojećoj React Native aplikaciji, ili čak izgradite društvenu mrežu!
---