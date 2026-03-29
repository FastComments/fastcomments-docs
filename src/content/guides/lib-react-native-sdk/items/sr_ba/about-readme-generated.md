---
Ova biblioteka je potpuna react-native implementacija [FastComments](https://fastcomments.com).

Podržava komentarisanje uživo, chat, thread-ove, emotikone, notifikacije, SSO, teme (skins) i potpunu prilagodbu slanjem objekta stilova. Svi resursi
takođe se mogu prilagoditi, i podržava prebacivanje različitih resursa na osnovu tamnog režima.

Prednost ove biblioteke je što je fleksibilnija od `fastcomments-react-native` wrapper-a. Komentari se renderuju pomoću nativnih komponenti umjesto unutar webview-a. Napomena: `react-native-webview` je i dalje potreban kao tranzitivna zavisnost uređivača bogatog teksta (`@10play/tentap-editor`).

Sve radi na FastComments backendu, tako da treba samo da integrišete UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Pogledajte [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) za više primjera.

Dodajte chat uživo svojoj postojećoj React Native aplikaciji, ili čak napravite društvenu mrežu!
---