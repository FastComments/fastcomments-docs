Ta knjižnica je popolna react-native implementacija [FastComments](https://fastcomments.com).

Podpira komentiranje v živo, klepet, niti, emotikone, obvestila, SSO, teme (skins) in popolno prilagoditev z posredovanjem objekta stylesheet. Vsi viri
so prav tako prilagodljivi, prav tako pa podpira menjavo različnih virov glede na temni način.

Prednost te knjižnice je, da je bolj prilagodljiva in ne zahteva webview, kot to počne ovitek `fastcomments-react-native`.

Vse teče na FastComments backendu, zato morate vključiti le uporabniški vmesnik:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Oglejte si [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) za več primerov.

Dodajte klepet v živo v vašo obstoječo React Native aplikacijo ali celo zgradite družbeno omrežje!