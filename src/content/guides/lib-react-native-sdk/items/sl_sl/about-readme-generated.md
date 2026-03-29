Ta knjižnica je popolna react-native implementacija [FastComments](https://fastcomments.com).

Podpira komentiranje v živo, klepet, niti, emotikone, obvestila, SSO, skins in popolno prilagoditev z uporabo objekta stylesheet. Vse assets je mogoče tudi prilagoditi, podprta pa je tudi preklapljanja različnih assets glede na temni način.

Prednost te knjižnice je v večji prilagodljivosti kot pri wrapperju `fastcomments-react-native`. Komentarji so upodobljeni z nativnimi komponentami namesto znotraj webviewa. Opomba: `react-native-webview` je še vedno potreben kot posredna odvisnost bogatega urejevalnika besedila (`@10play/tentap-editor`).

Vse teče na FastComments backendu, zato morate vključiti le UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

Za več primerov si oglejte [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src).

Dodajte klepet v živo v obstoječo React Native aplikacijo ali pa zgradite družbeno omrežje!