Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Uživo komentiranje</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Uživo komentiranje, svijetla tema"/></td>
    <td align="center"><b>Tamna tema</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Uživo komentiranje, tamna tema"/></td>
    <td align="center"><b>Uživo chat</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Predložak uživo chata"/></td>
  </tr>
</table>

### Rich Text Editor

Ova biblioteka koristi [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) za uređivanje bogatog teksta, što pruža snažno WYSIWYG iskustvo uređivanja. Isti editor pokreće iOS, Android i web (putem `react-native-web`), pa se sastavljač ponaša dosljedno na svim platformama s jednom implementacijom.

`react-native-enriched` zahtijeva React Native New Architecture (Fabric) na nativnoj strani (zadano od RN 0.76, opcionalno na RN 0.72-0.75), i bundler koji rješava uvjete `exports` paketa. Ovaj SDK je razvijan i testiran protiv RN 0.81 / React 19. Isti editor također radi na webu preko `react-native-web`; web build enriched editora je i dalje označen kao eksperimentalni upstream.

### Widgets

SDK dolazi s tri widgeta, koji prate FastComments Android SDK:

- `FastCommentsLiveCommenting` - uvlaknuto komentiranje s glasovima, odgovorima, paginacijom, spominjanjima, obavijestima i live ažuriranjima.
- `FastCommentsLiveChat` - predložak chata na istom mehanizmu: kronološke poruke s novima na dnu, sastavljač ispod liste, live zaglavni trak (točka veze + broj korisnika), beskonačna povijest učitava se skrolanjem prema gore, automatsko skrolanje na nove poruke, bez glasova ili threading odgovora. Svaki predložak može se prebrisati putem `config`.
- `FastCommentsFeed` - društveni feed s komponiranjem objava, medijima, reakcijama, praćenjima i live trakama za nove objave.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

Zadani izgled generira se iz skupa semantičkih dizajnerskih tokena (`FastCommentsTheme`): boje, razmaci, radijusi, veličine fonta, debljine fonta i veličine avatara. Proslijedite djelomične preinake tokena (tipizirano `FastCommentsThemeOverrides`) putem `theme` prop-a na bilo kojem widgetu i čitavo stablo stilova će se dosljedno restilirati:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Tamni način rada je udaljen samo jedan token set:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

Prop `styles` i dalje prihvaća sirovo `IFastCommentsStyles` stablo za kiruršku kontrolu. Kada su `theme` i `styles` oba pružena, eksplicitni stilovi nadjačavaju temirano stablo; kada je samo `styles` pružen, on u potpunosti zamjenjuje zadane vrijednosti (izvorno ponašanje, tako da postojeće integracije i skinovi nisu pogođeni). `setupDarkModeSkin` je zastario u korist `theme` propa.

### Configuration Options

Ova biblioteka nastoji podržati sve opcije konfiguracije definirane u [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), isto kao i web implementacija.

Osim toga, React Native dodaje nekoliko specifičnih opcija SDK-a putem `FastCommentsRNConfig`:

- `hideTopBar` - sakrije trak s prijavljenim korisnikom / zvonom za obavijesti prikazan iznad sastavljača.
- `usePressToEdit` - pritisnite i držite komentar da otvorite njegov izbornik.
- `disableDownVoting` - sakrije gumbe za negativno glasovanje.
- `renderCommentInline` - prikaže informacije o komentatoru unutar istog HTML bloka kao i sadržaj komentara.
- `renderLikesToRight` - pomakne područje glasova/likeova na desnu stranu komentara umjesto ispod njega.
- `renderDateBelowComment` - prikaže datum ispod komentara.
- `showLiveStatus` - prikaže chat-stil "Live" + zaglavni trak s brojem korisnika iznad komentara.
- `useInlineSubmitButton` - prikaže gumb za slanje kao ikonu unutar sastavljača.
- `countAboveToggle` - s `useShowCommentsToggle`, koliko komentara se prikazuje iznad "Show Comments" preklopke.
- `preserveFeedScrollPosition` - `FastCommentsFeed` pamti svoj scroll offset preko unmount/remount (zadano true).

### FastComments Concepts

Glavni koncepti koje treba znati za početak su `tenantId` i `urlId`. `tenantId` je identifikator vašeg FastComments.com računa. `urlId` je za što će biti vezani threadovi komentara. To može biti URL stranice, ili id proizvoda, id članka, itd.

### User Notifications

FastComments podržava obavijesti za [mnoge scenarije](https://docs.fastcomments.com/guide-notifications.html). Obavijesti su konfigurabilne,
mogu se isključiti globalno ili na razini obavijesti/komentara, i podržava pretplate na razini stranice tako da se korisnici mogu pretplatiti na threadove određene
stranice ili članka.

Na primjer, moguće je koristiti Secure SSO za autentikaciju korisnika i tada periodično poll-ati nepročitane obavijesti i gurnuti ih korisniku.

Pogledajte [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za primjer kako dobiti i prevesti nepročitane korisničke obavijesti.

### Gif Browser

Po zadanim postavkama, odabir slika ili gifova nije omogućen. Pogledajte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za način
kako podržati upload slika i gifova. U ovoj biblioteci postoji Gif Browser koji anonimizira pretrage i slike koje su pružene, dovoljno ga je samo koristiti.

### Performance

Otvorite ticket s primjerom za reprodukciju, uključujući uređaj koji ste koristili, ako identificirate bilo kakve probleme s performansama. Performanse su prioritet
u svim FastComments bibliotekama.