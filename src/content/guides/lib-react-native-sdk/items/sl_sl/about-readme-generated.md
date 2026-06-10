Ta knjižnica je popolna implementacija za react-native [FastComments](https://fastcomments.com).

Podpira komentiranje v živo, klepet, niti (threads), emotikone, obvestila, SSO, teme (skins) in popolno prilagoditev z posredovanjem objekta s slogi. Vse vire (assets) je mogoče tudi prilagoditi, podpira pa preklapljanje med različnimi viri glede na temni način (dark mode).

Prednost te knjižnice je, da je bolj prilagodljiva kot ovojnica `fastcomments-react-native`. Komentarji so upodobljeni z nativnimi komponentami namesto znotraj webviewa.

Vse deluje na FastComments backendu, zato morate vključiti le uporabniški vmesnik (UI):

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Oglejte si [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) za več primerov.

Dodajte klepet v živo v vašo obstoječo aplikacijo React Native, ali pa celo zgradite družbeno omrežje!