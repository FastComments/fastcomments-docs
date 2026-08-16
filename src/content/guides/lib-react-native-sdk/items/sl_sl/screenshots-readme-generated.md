Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Živo komentiranje</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Živo komentiranje, svetla tema"/></td>
    <td align="center"><b>Temna tema</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Živo komentiranje, temna tema"/></td>
    <td align="center"><b>Klepet v živo</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Prednastavitev klepeta v živo"/></td>
  </tr>
</table>

### Urejevalnik bogatega besedila

Ta knjižnica uporablja [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) za urejanje bogatega besedila, ki omogoča zmogljivo izkušnjo urejanja WYSIWYG. Enak urejevalnik poganja iOS, Android in splet (prek `react-native-web`), zato se sestavljalnik obnaša dosledno na vseh platformah z eno samo implementacijo.

`react-native-enriched` zahteva novo arhitekturo React Native (Fabric) na nativnih platformah (privzeto od RN 0.76, možnost vklopa na RN 0.72‑0.75) in paketnik, ki razrešuje pogoje `exports` paketa. Ta SDK je razvit in preizkušen na RN 0.81 / React 19. Enak urejevalnik deluje tudi na spletu prek `react-native-web`; spletna različica enriched urejevalnika je še vedno označena kot eksperimentalna v izvoru.

### Gradniki

SDK vsebuje tri gradnike, ki odražajo FastComments Android SDK:

- `FastCommentsLiveCommenting` – nitkasto komentiranje z glasovanjem, odgovori, paginacijo, omembami, obvestili in živimi posodobitvami.
- `FastCommentsLiveChat` – prednastavitev klepeta na istem motorju: kronološka sporočila z novimi na dnu, sestavljalnik pod seznamom, živa vrstica glave (pika povezave + število uporabnikov), neskončna zgodovina, ki se naloži s pomikanjem navzgor, samodejno pomikanje do novih sporočil, brez glasov ali nitkastega odgovarjanja. Vsako prednastavitev je mogoče prepisati prek `config`.
- `FastCommentsFeed` – socialni vir s sestavljalnikom objav, mediji, reakcijami, sledenjem in živimi pasicami za nove objave.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Tematizacija

Privzeti videz je ustvarjen iz nabora semantičnih oblikovnih žetonov (`FastCommentsTheme`): barve, razmiki, polmer, velikosti pisav, teže pisav in velikosti avatarjev. Posredujte delne preglasitve žetonov (tipa `FastCommentsThemeOverrides`) prek lastnosti `theme` na kateremkoli gradniku in celotno drevo stilov se dosledno preoblikuje:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Temni način je le en nabor žetonov stran:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

Lastnost `styles` še vedno sprejema surovo drevo `IFastCommentsStyles` za natančen nadzor. Ko sta podani tako `theme` kot `styles`, eksplicitni stili preglasijo tematsko drevo; ko je podana le `styles`, ta popolnoma nadomesti privzete (izvorno vedenje, zato obstoječe integracije in preobleke ostanejo nedotaknjene). `setupDarkModeSkin` je zastarel v korist lastnosti `theme`.

### Možnosti konfiguracije

Ta knjižnica si prizadeva podpirati vse možnosti konfiguracije, definirane v [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), tako kot spletna implementacija.

Poleg tega React Native doda nekaj SDK-specifičnih možnosti prek `FastCommentsRNConfig`:

- `hideTopBar` – skrije vrstico z vpisanim uporabnikom / zvoncem za obvestila, prikazano nad sestavljalnikom.
- `usePressToEdit` – pritisni in drži komentar, da odpreš njegov meni.
- `disableDownVoting` – skrije gumbe za negativno glasovanje.
- `renderCommentInline` – prikaže informacije o komentatorju znotraj istega HTML bloka kot vsebina komentarja.
- `renderLikesToRight` – premakne območje glasov/ všečkov na desno stran komentarja namesto pod njim.
- `renderDateBelowComment` – prikaže datum pod komentarjem.
- `showLiveStatus` – prikaže vrstico glave v slogu klepeta "Live" + število uporabnikov nad komentarji.
- `useInlineSubmitButton` – prikaže gumb za pošiljanje kot ikono znotraj sestavljalnika.
- `countAboveToggle` – skupaj z `useShowCommentsToggle`, koliko komentarjev se prikaže nad preklopom "Show Comments".
- `preserveFeedScrollPosition` – `FastCommentsFeed` si zapomni svoj pomik pomikanja med odmontiranjem/ponovnim montažom (privzeto true).

### FastComments koncepti

Glavni koncepti, ki jih je treba poznati za začetek, so `tenantId` in `urlId`. `tenantId` je identifikator vašega računa na FastComments.com. `urlId` je mesto, na katerega so vezane nitke komentarjev. To je lahko URL strani, ID izdelka, ID članka itd.

### Lokalizacija

Vse besedilo, ki ga vidijo uporabniki v teh gradnikih (oznake gumbov, nadomestna besedila, prazna stanja, relativni datumi kot "5 minut nazaj", sporočila o napakah itd.) je **strežnikovno vodeno**. Komponente ne vsebujejo trdno zakodiranih angleških nizov; prikazujejo prevode, ki jih FastComments zagotavlja za zahtevano lokalno nastavitev.

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

FastComments podpira obvestila za [mnogo scenarijev](https://docs.fastcomments.com/guide-notifications.html). Obvestila so nastavljiva, se lahko globalno ali na ravni obvestila/komentarja izključijo, ter podpirajo naročnine na ravni strani, tako da se uporabniki lahko naročijo na nitke določene strani ali članka.

Na primer, mogoče je uporabiti Secure SSO za overitev uporabnika in nato periodično preverjati neprebrana obvestila ter jih pošiljati uporabniku.

Oglejte si [primer AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za način pridobivanja in prevajanja neprebranih uporabniških obvestil.

### Brskalnik GIF-ov

Privzeto ni omogočenega izbora slik ali GIF-ov. Oglejte si [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za način podpiranja nalaganja slik in GIF-ov. Obstaja brskalnik GIF-ov, ki anonimizira iskanja in slike, ki jih ponuja ta knjižnica; preprosto ga morate uporabiti.

### Učinkovitost

Prosimo, odprite zahtevek z vzorcem za reprodukcijo, vključno z uporabo naprave, če opazite kakršnekoli težave z učinkovitostjo. Učinkovitost je ključni del vseh FastComments knjižnic.