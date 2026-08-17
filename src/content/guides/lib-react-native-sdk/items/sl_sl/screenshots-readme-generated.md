Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Živo komentarjenje</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Živo komentarjenje, svetla tema"/></td>
    <td align="center"><b>Temna tema</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Živo komentarjenje, temna tema"/></td>
    <td align="center"><b>Živo klepet</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Prednastavitev za živo klepet"/></td>
  </tr>
</table>

### Urejevalnik bogatega besedila

Ta knjižnica uporablja [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) za urejanje bogatega besedila, ki nudi zmogljivo izkušnjo urejanja WYSIWYG. Enak urejevalnik poganja iOS, Android in splet (prek `react-native-web`), zato se sestavljalnik obnaša dosledno na vseh platformah z eno samo implementacijo.

`react-native-enriched` zahteva novo arhitekturo React Native (Fabric) na native (privzeto od RN 0.76, možnost vklopa na RN 0.72-0.75) in paketni upravljalnik, ki razrešuje pogoje `exports` paketa. Ta SDK je razvit in preizkušen proti RN 0.81 / React 19. Enak urejevalnik deluje tudi na spletu prek `react-native-web`; spletna različica enriched urejevalnika je še vedno označena kot eksperimentalna pri izvoru.

### Gradniki

SDK vsebuje tri gradnike, ki odražajo FastComments Android SDK:

- `FastCommentsLiveCommenting` - komentarjenje v nitih z glasovi, odgovori, paginacijo, omembami, obvestili in živimi posodobitvami.
- `FastCommentsLiveChat` - prednastavitev klepeta na istem motorju: kronološka sporočila z novimi na dnu, sestavljalnik pod seznamom, živa vrstica glave (pika povezave + število uporabnikov), neskončna zgodovina, ki se naloži s pomikanjem navzgor, samodejno pomikanje do novih sporočil, brez glasov ali nitnega odgovarjanja. Vsako prednastavitev je mogoče prepisati prek `config`.
- `FastCommentsFeed` - družbeni vir z sestavljalnikom objav, mediji, reakcijami, sledenji in živimi pasicami za nove objave.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Tematizacija

Privzeti izgled je ustvarjen iz nabora semantičnih oblikovnih žetonov (`FastCommentsTheme`): barve, razmike, radij, velikosti pisav, teže pisav in velikosti avatarjev. Posredujte delne preglasitve žetonov (tipa `FastCommentsThemeOverrides`) prek lastnosti `theme` na kateremkoli gradniku in celotno drevo stilov se bo dosledno preoblikovalo:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Temni način je le en nabor žetonov stran:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

Lastnost `styles` še vedno sprejema surovo drevo `IFastCommentsStyles` za natančen nadzor. Ko sta podani tako `theme` kot `styles`, eksplicitni slogi presegajo tematsko drevo; ko je podana le `styles`, ta popolnoma nadomesti privzete (izvorno vedenje, zato obstoječe integracije in preobleke ostanejo nespremenjene). `setupDarkModeSkin` je zastarel v korist lastnosti `theme`.

### Možnosti konfiguracije

Ta knjižnica si prizadeva podpirati vse možnosti konfiguracije, definirane v [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), tako kot spletna implementacija.

Poleg tega React Native doda nekaj SDK-specifičnih možnosti prek `FastCommentsRNConfig`:

- `hideTopBar` - skrij vrstico z prijavljenim uporabnikom / zvoncem za obvestila, ki je prikazana nad sestavljalnikom.
- `usePressToEdit` - pritisni in drži komentar, da odpreš njegov meni.
- `disableDownVoting` - skrij gumbe za negativno glasovanje.
- `renderCommentInline` - prikaži informacije o komentatorju znotraj istega HTML bloka kot vsebina komentarja.
- `renderLikesToRight` - premakni območje glasov/ljubkov na desno stran komentarja namesto pod njim.
- `renderDateBelowComment` - prikaži datum pod komentarjem.
- `showLiveStatus` - prikaži vrstico glave v slogu klepeta "Live" + število uporabnikov nad komentarji.
- `useInlineSubmitButton` - prikaži gumb za pošiljanje kot ikono znotraj sestavljalnika.
- `countAboveToggle` - z `useShowCommentsToggle`, koliko komentarjev se prikaže nad preklopom "Pokaži komentarje".
- `preserveFeedScrollPosition` - `FastCommentsFeed` si zapomni svoj pomik drsenja med odmontiranjem/ponovnim montažom (privzeto true).

### Koncepti FastComments

Glavni koncepti, ki jih je treba poznati za začetek, so `tenantId` in `urlId`. `tenantId` je vaš identifikator računa na FastComments.com. `urlId` je mesto, na katerega so vezane nitke komentarjev. To je lahko URL strani, ID izdelka, ID članka itd.

### Lokalizacija

Vse besedilo, ki ga vidijo uporabniki v teh gradnikih (oznake gumbov, nadomestna besedila, prazna stanja, relativni datumi kot "5 minut nazaj", sporočila o napakah itd.) je **strežnikovno vodeno**. Komponente ne vsebujejo trdno zakodiranih angleških nizov; prikazujejo prevode, ki jih FastComments ponuja za zahtevano lokalno nastavitev.

Za zahtevo lokalne nastavitve nastavite `locale` v vaši konfiguraciji:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Ko `locale` ni nastavljen, FastComments ponudi privzeti jezik najemnika.

**Urejanje besedila:** prevodi se upravljajo v vašem nadzorni plošči FastComments, ne v tem SDK-ju. Za spremembo besedila prepišite privzeto besedilo ali dodajte jezik, uredite prevode za vaš račun v nadzorni plošči – sprememba se samodejno ujame v gradnikih brez potrebe po izidu aplikacije. SDK ne vsebuje angleških nadomestkov, zato vsak ključ, ki ga izpraznite v nadzorni plošči, prikaže prazno; ohranite ključe izpolnjene za vsako podprto lokalno nastavitev.

### Obvestila uporabnikov

FastComments podpira obvestila za [mnogo scenarijev](https://docs.fastcomments.com/guide-notifications.html). Obvestila so nastavljiva, lahko se jih globalno ali na ravni obvestila/komentarja izključi, in podpirajo naročnine na ravni strani, tako da se uporabniki lahko naročijo na niti določene strani ali članka.

Na primer, mogoče je uporabiti Secure SSO za overitev uporabnika in nato periodično preverjati neprebrana obvestila ter jih pošiljati uporabniku.

Oglejte si [primer AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za način pridobivanja in prevajanja neprebranih uporabniških obvestil.

### Brskalnik GIF-ov

Privzeto ni omogočenega izbira slik ali GIF-ov. Oglejte si [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za način podpiranja nalaganja slik in GIF-ov. Obstaja brskalnik GIF-ov, ki anonimizira iskanja in slike, ki jih ponuja ta knjižnica; preprosto ga morate uporabiti.

### Zmogljivost

Prosimo, odprite zahtevek z primerom za reprodukcijo, vključno z uporabljenim napravo, če opazite kakršnekoli težave z zmogljivostjo. Zmogljivost je ključni del vseh FastComments knjižnic.