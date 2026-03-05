Ova biblioteka je kompletna react-native implementacija [FastComments](https://fastcomments.com).

Podržava live komentarisanje, chat, thread-ove, emotikone, notifikacije, SSO, skinove i potpunu prilagodbu prosleđivanjem objekta sa stilovima. Svi resursi
takođe se mogu prilagoditi, i podržava prebacivanje različitih resursa u zavisnosti od tamnog režima.

Prednost ove biblioteke je što je fleksibilnija i ne zahteva webview, kao `fastcomments-react-native` wrapper.

Sve to radi na FastComments backend-u, tako da treba samo da integrišete UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Pogledajte [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) za više primera.

Dodajte chat uživo vašoj postojećoj React Native aplikaciji, ili čak izgradite društvenu mrežu!