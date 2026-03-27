---
Ova biblioteka je potpuna react-native implementacija [FastComments](https://fastcomments.com).

Podržava komentare uživo, chat, threadove, emotikone, notifikacije, SSO, teme, i potpunu prilagodbu prosleđivanjem objekta stilova. Svi resursi
takođe se mogu prilagoditi, i podržano je prebacivanje različitih resursa u zavisnosti od tamnog režima.

Prednost ove biblioteke je što je fleksibilnija od `fastcomments-react-native` wrapper-a. Komentari se prikazuju pomoću nativnih komponenti umesto unutar webview-a. Napomena: `react-native-webview` je i dalje potreban kao transitivna zavisnost editora za bogati tekst (`@10play/tentap-editor`).

Sve se pokreće na FastComments backend-u, tako da jedino treba da integrišete UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Pogledajte [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) za više primera.

Dodajte chat uživo u vašu postojeću React Native aplikaciju, ili čak izgradite društvenu mrežu!
---