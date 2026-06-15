Živo nitasto komentiranje z avatarji, gnezdenimi odgovori, glasovanjem in vgrajenim urejevalnikom obogatenega besedila, poleg temne teme in prednastavitve za živo klepetanje (tukaj prikazano renderirano preko `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Živo komentiranje</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Živo komentiranje, svetla tema"/></td>
    <td align="center"><b>Temna tema</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Živo komentiranje, temna tema"/></td>
    <td align="center"><b>Klepet v živo</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Prednastavitev klepeta v živo"/></td>
  </tr>
</table>

### Urejevalnik obogatenega besedila

Ta knjižnica uporablja [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) za urejanje obogatenega besedila, kar omogoča zmogljivo WYSIWYG izkušnjo urejanja. Isti urejevalnik poganja iOS, Android in splet (prek `react-native-web`), zato se komponist obnaša dosledno na vseh platformah z eno samo implementacijo.

`react-native-enriched` zahteva React Native New Architecture (Fabric) na native (privzeto od RN 0.76, na RN 0.72-0.75 je potrebna privolitev), in bundler, ki rešuje pogoje `exports` paketov. Ta SDK je razvit in preizkušen proti RN 0.81 / React 19. Isti urejevalnik teče tudi na spletu skozi `react-native-web`; spletna gradnja obogatenega urejevalnika je še vedno označena kot eksperimentalna v upstream.

### Gradniki

SDK vključuje tri gradnike, ki odražajo FastComments Android SDK:

- `FastCommentsLiveCommenting` - nitasto komentiranje z glasovanji, odgovori, paginacijo, omembami, obvestili in živimi posodobitvami.
- `FastCommentsLiveChat` - prednastavitev klepeta na istem pogonu: kronološka sporočila z novimi na dnu, komponist pod seznamom, živa zgornja vrstica (pika povezave + število uporabnikov), neskončna zgodovina naložena s pomikanjem navzgor, samodejno pomikanje do novih sporočil, brez glasov ali razvejanih odgovorov. Vsako prednastavitev je mogoče prepisati preko `config`.
- `FastCommentsFeed` - družbeni feed z urejevalnikom objav, mediji, reakcijami, sledenji in pasicami za nove objave v živo.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Teme

Privzeti videz je ustvarjen iz niza semantičnih oblikovnih tokenov (`FastCommentsTheme`): barve, razmiki, radiji, velikosti pisav, debeline pisav in velikosti avatarjev. Posredujte delne override-e tokenov (tipizirano `FastCommentsThemeOverrides`) preko prop-a `theme` na kateremkoli gradniku in celotno drevo slogov se dosledno preoblikuje:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Temni način je le en komplet tokenov stran:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

Prop `styles` še vedno sprejema surovo drevo `IFastCommentsStyles` za natančen nadzor. Ko sta `theme` in `styles` oba podana, prevladajo eksplicitni slogi nad tematiziranim drevesom; ko je podan samo `styles`, popolnoma nadomesti privzete sloge (originalno vedenje, tako da obstoječe integracije in skin-i niso prizadeti). `setupDarkModeSkin` je zastarel v prid prop-u `theme`.

### Možnosti konfiguracije

Ta knjižnica si prizadeva podpreti vse možnosti konfiguracije, definirane v [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), enako kot spletna implementacija.

Poleg teh, React Native doda nekaj SDK-specifičnih možnosti preko `FastCommentsRNConfig`:

- `hideTopBar` - skrije vrstico prijavljenega uporabnika / ikone obvestil, prikazano nad komponistom.
- `usePressToEdit` - pritisnite in pridržite komentar za odpiranje njegovega menija.
- `disableDownVoting` - skrije gumbe za negativno glasovanje.
- `renderCommentInline` - prikaže podatke o komentarju znotraj istega HTML bloka kot vsebina komentarja.
- `renderLikesToRight` - premakne območje glasovanja/všečkov desno od komentarja namesto pod njim.
- `renderDateBelowComment` - prikaže datum pod komentarjem.
- `showLiveStatus` - prikaže klepetu podoben zgornji trak 'Live' + število uporabnikov nad komentarji.
- `useInlineSubmitButton` - prikaže gumb za oddajo kot ikono znotraj komponista.
- `countAboveToggle` - z `useShowCommentsToggle`, koliko komentarjev se prikaže nad preklopom 'Show Comments'.
- `preserveFeedScrollPosition` - `FastCommentsFeed` si zapomni svoj pomik skozi unmount/remount (privzeto true).

### Pojmi FastComments

Glavni pojmi, ki jih morate poznati za začetek, so `tenantId` in `urlId`. `tenantId` je identifikator vašega FastComments.com računa. `urlId` je mesto, s katerim bodo nitke komentarjev povezane. To je lahko URL strani, id produkta, id članka itd.

### Obvestila uporabnikom

FastComments podpira obvestila za [mnoge scenarije](https://docs.fastcomments.com/guide-notifications.html). Obvestila so konfigurabilna, jih je mogoče preklicati globalno ali na ravni obvestila/komentarja, in podpirajo naročnine na ravni strani, tako da se lahko uporabniki naročijo na nitke določene strani ali članka.

Na primer, mogoče je uporabiti Secure SSO za avtentikacijo uporabnika in nato periodično preverjati neprebrana obvestila ter jih posredovati uporabniku.

Oglejte si [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za to, kako pridobiti in prevesti neprebrana uporabniška obvestila.

### Brskalnik GIF-ov

Privzeto ni omogočena izbira slik ali gifov. Oglejte si [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za način podpore nalaganja slik in gifov. V tej knjižnici je na voljo Brskalnik GIF-ov, ki anonimizira iskanja in slike, ki jih zagotavlja knjižnica — preprosto ga morate uporabljati.

### Zmogljivost

Če opazite težave z zmogljivostjo, odprite ticket z enim primerom za reprodukcijo, vključno z napravo, ki je bila uporabljena. Zmogljivost je prvorazredna prioriteta vseh FastComments knjižnic.