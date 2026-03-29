Ova biblioteka je potpuna react-native implementacija [FastComments](https://fastcomments.com).

Podržava komentarisanje uživo, chat, thread-ove, emotikone, notifikacije, SSO, skinove, i potpunu prilagodbu prosljeđivanjem objekta sa stilovima. Svi asseti
mogu takođe biti prilagođeni, i podržava prebacivanje različitih asseta u zavisnosti od tamnog režima.

Prednost ove biblioteke je što je fleksibilnija od `fastcomments-react-native` wrapper-a. Komentari se renderuju pomoću nativnih komponenti umjesto unutar webview-a. Napomena: `react-native-webview` je i dalje potreban kao transitivna zavisnost rich text editora (`@10play/tentap-editor`).

Sve radi na FastComments backend-u, tako da treba da integrišete samo UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Pogledajte [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) za više primjera.

Dodajte chat uživo u vašu postojeću React Native aplikaciju, ili čak izgradite društvenu mrežu!