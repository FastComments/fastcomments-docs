Ova biblioteka je potpuna react-native implementacija [FastComments](https://fastcomments.com).

Podržava komentarisanje uživo, chat, thread-ove, emotikone, obaveštenja, SSO, skinove i potpunu prilagodbu prosleđivanjem stylesheet objekta. Svi asseti se takođe mogu prilagoditi, i podržano je prebacivanje različitih asseta na osnovu tamnog režima.

Prednost ove biblioteke je u tome što je fleksibilnija od `fastcomments-react-native` wrappera. Komentari se renderuju pomoću nativnih komponenti umesto unutar webview-a.

Sve to radi na FastComments backendu, tako da vi morate samo da integrišete UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Pogledajte [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) za više primera.

Dodajte chat uživo vašoj postojećoj React Native aplikaciji, ili čak napravite društvenu mrežu!