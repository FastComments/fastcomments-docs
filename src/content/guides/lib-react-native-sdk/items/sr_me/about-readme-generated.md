Ova biblioteka је potpuna react-native implementacija [FastComments](https://fastcomments.com).

Podržava komentarisanje uživo, chat, threads, emoticons, notifikacije, SSO, skins, и potpunu prilagodbu prosleđivanjem stylesheet objekta. Svi assets такође се могу prilagoditi, и podržano je prebacivanje različitih assets na osnovu dark mode.

Prednost ove biblioteke је што је fleksibilnija, и ne захтијева webview, као `fastcomments-react-native` wrapper.

Sve ради на FastComments backendu, тако да треба да integrišete samo UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Pogledajte [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) за више примјера.

Dodajte live chat у вашу постојећу React Native апликацију, или чак изградите друштвену мрежу!