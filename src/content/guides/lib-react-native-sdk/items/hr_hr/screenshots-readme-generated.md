Live threaded komentiranje s avatarima, ugniježdenim odgovorima, glasovima i ugrađenim uređivačem bogatog teksta, plus tamna tema i predložak za live‑chat (prikazano ovdje renderirano putem `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live Commenting</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Live commenting, light theme"/></td>
    <td align="center"><b>Dark Theme</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Live commenting, dark theme"/></td>
    <td align="center"><b>Live Chat</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Live chat preset"/></td>
  </tr>
</table>

### Rich Text Editor

Ova biblioteka koristi [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) za uređivanje bogatog teksta, što pruža snažno WYSIWYG iskustvo uređivanja. Isti uređivač pokreće iOS, Android i web (preko `react-native-web`), tako da se sastavljač ponaša dosljedno na svim platformama uz jednu implementaciju.

`react-native-enriched` zahtijeva React Native New Architecture (Fabric) na native (zadano od RN 0.76, opcionalno od RN 0.72‑0.75) i bundler koji rješava `exports` uvjete paketa. Ovaj SDK je razvijen i testiran protiv RN 0.81 / React 19. Isti uređivač također radi na webu putem `react-native-web`; web‑build enriched uređivača još uvijek je označen kao eksperimentalni upstream.

### Widgets

SDK isporučuje tri widgeta, u skladu s FastComments Android SDK‑om:

- `FastCommentsLiveCommenting` – ugniježdeno komentiranje s glasovima, odgovorima, paginacijom, spominjanjima, obavijestima i live ažuriranjima.
- `FastCommentsLiveChat` – predložak za chat baziran na istom motoru: kronološke poruke s novim na dnu, sastavljač ispod popisa, live zaglavlje (točka veze + broj korisnika), beskonačna povijest učitana skrolanjem prema gore, automatsko pomicanje na nove poruke, bez glasova ili ugniježdenih odgovora. Svaki predložak može se prepisati putem `config`.
- `FastCommentsFeed` – društveni feed s sastavljačem objava, medijima, reakcijama, praćenjima i live bannerima za nove objave.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

Zadani izgled generira se iz skupa semantičkih dizajnerskih tokena (`FastCommentsTheme`): boje, razmaci, radijusi, veličine fonta, težine fonta i veličine avatara. Proslijedite djelomična prepisivanja tokena (tipa `FastCommentsThemeOverrides`) kroz `theme` prop na bilo kojem widgetu i cijelo stablo stilova će se dosljedno preoblikovati:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Tamni način rada je samo jedan set tokena udaljen:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` prop i dalje prihvaća sirovo `IFastCommentsStyles` stablo za kirursku kontrolu. Kada su `theme` i `styles` oba dostavljena, izričiti stilovi imaju prednost nad tematskim stablom; kada je dostavljen samo `styles`, on u potpunosti zamjenjuje zadane (originalno ponašanje, pa postojeće integracije i skinovi nisu pogođeni). `setupDarkModeSkin` je zastarjelo u korist `theme` prop‑a.

### Configuration Options

Ova biblioteka nastoji podržati sve opcije konfiguracije definirane u [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), baš kao i web implementacija.

Uz to, React Native dodaje nekoliko SDK‑specifičnih opcija putem `FastCommentsRNConfig`:

- `hideTopBar` – sakrij traku s prijavljenim korisnikom / zvono obavijesti iznad sastavljača.
- `usePressToEdit` – pritisni‑i‑drži komentar da otvoriš njegov izbornik.
- `disableDownVoting` – sakrij gumbe za negativno glasanje.
- `renderCommentInline` – prikaži informacije o komentatoru unutar istog HTML bloka kao i sadržaj komentara.
- `renderLikesToRight` – premjesti područje glasova/​lajkova desno od komentara umjesto ispod.
- `renderDateBelowComment` – prikaži datum ispod komentara.
- `showLiveStatus` – prikaži chat‑stil „Live“ + traku s brojem korisnika iznad komentara.
- `useInlineSubmitButton` – prikaži gumb za slanje kao ikonu unutar sastavljača.
- `countAboveToggle` – uz `useShowCommentsToggle`, koliko komentara se prikazuje iznad prekidača „Show Comments“.
- `preserveFeedScrollPosition` – `FastCommentsFeed` pamti svoj pomak skrolanja kroz unmount/remount (zadano true).

### FastComments Concepts

Glavni pojmovi o kojima treba znati za početak su `tenantId` i `urlId`. `tenantId` je vaš identifikator računa na FastComments.com. `urlId` je onaj na koji će se povezati niti komentara. To može biti URL stranice, ID proizvoda, ID članka, itd.

### Localization

Sav tekst koji korisnici vide u ovim widgetima (oznake gumba, placeholderi, prazna stanja, relativni datumi poput „5 minutes ago“, poruke o greškama, itd.) je **server‑driven**. Komponente ne hard‑kodiraju engleske stringove; one renderiraju prijevode koje FastComments pruža za traženi jezik.

Za zahtjev za jezik, postavite `locale` u vašoj konfiguraciji:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Kada `locale` nije postavljen, FastComments servira zadan jezik najstamparije.

**Uređivanje teksta:** prijevodi se upravljaju u vašoj FastComments nadzornoj ploči, a ne u ovom SDK‑u. Za promjenu formulacije, prepišite zadani tekst ili dodajte jezik, uredite prijevode za vaš račun u nadzornoj ploči – promjena se automatski primijeni na widgete bez potrebe za novim izdanjem aplikacije. SDK ne isporučuje engleske rezervne vrijednosti, pa svaki ključ koji ostavite praznim u nadzornoj ploči prikazuje se prazno; održavajte ključeve popunjene za svaki jezik koji podržavate.

### User Notifications

FastComments podržava obavijesti za [mnogo scenarija](https://docs.fastcomments.com/guide-notifications.html). Obavijesti su konfigurabilne, mogu se globalno ili na razini obavijesti/​komentara isključiti, i podržavaju pretplate na razini stranice tako da korisnici mogu pretplatiti se na niti određene stranice ili članka.

Na primjer, moguće je koristiti Secure SSO za autentifikaciju korisnika, a zatim periodično provjeravati nepročitane obavijesti i slati ih korisniku.

Pogledajte [primjer AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za **kako** dobiti i **prevesti** nepročitane korisničke obavijesti.

### Gif Browser

Prema zadanim postavkama, odabir slika ili gif‑ova nije omogućen. Pogledajte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za to kako podržati učitavanje slika i gif‑ova. Postoji Gif Browser koji anonimno pretražuje i pruža slike u ovoj biblioteci; jednostavno ga koristite.

### Performance

Molimo otvorite ticket s primjerom za reprodukciju, uključujući korišteni uređaj, ako uočite bilo koji problem s performansama. Performanse su prioritetna stavka u svim FastComments bibliotekama.