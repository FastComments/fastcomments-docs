Live threaded komentarisanje sa avatarima, ugniježdenim odgovorima, glasovima i ugrađenim rich‑text komponantom, plus tamna tema i preset za live‑chat (prikazano ovdje renderovano putem `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Live komentar</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Live komentar, svijetla tema"/></td>
    <td align="center"><b>Tamna tema</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Live komentar, tamna tema"/></td>
    <td align="center"><b>Live chat</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Preset za live chat"/></td>
  </tr>
</table>

### Rich Text Editor

Ova biblioteka koristi [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) za rich‑text uređivanje, koji pruža moćno WYSIWYG iskustvo uređivanja. isti editor pokreće iOS, Android i web (preko `react-native-web`), pa se komponant ponaša dosljedno na svim platformama uz jedinstvenu implementaciju.

`react-native-enriched` zahtijeva React Native New Architecture (Fabric) na native (default od RN 0.76, opcionalno na RN 0.72‑0.75), i bundler koji rješava uslove `exports` paketa. Ovaj SDK je razvijen i testiran protiv RN 0.81 / React 19. isti editor također radi na webu kroz `react-native-web`; web‑build enrich‑ed editora je i dalje označen kao eksperimentalni upstream.

### Widgets

SDK isporučuje tri widgeta, koji preslikavaju FastComments Android SDK:

- `FastCommentsLiveCommenting` – komentarisanje u niti sa glasovima, odgovorima, paginacijom, spominjanjima, notifikacijama i live ažuriranjima.
- `FastCommentsLiveChat` – chat preset nad istim engine‑om: hronološke poruke s novim na dnu, komponant ispod liste, traka zaglavlja (dot za konekciju + broj korisnika), beskonačna historija učitana skrolovanjem gore, automatsko skrolovanje na nove poruke, bez glasova i ugniježđivanja odgovora. Svaki preset može biti prepisan putem `config`.
- `FastCommentsFeed` – društveni feed s komponantom za objavu, medijima, reakcijama, praćenjima i live bannerima za nove objave.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

Podrazumijevani izgled generiše se iz skupa semantičkih dizajnerskih tokena (`FastCommentsTheme`): boje, razmaci, radius, veličine fonta, debljine fonta i veličine avatara. Proslijedite parcijalna nadjačavanja tokena (tipizirano `FastCommentsThemeOverrides`) kroz `theme` prop na bilo kojem widgetu i cijelo stablo stilova će se dosljedno prestilizovati:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Tamni režim je samo jedan set tokena udaljen:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

Prop `styles` i dalje prihvata raw `IFastCommentsStyles` stablo za kiruršku kontrolu. Kada su `theme` i `styles` oba proslijeđena, eksplicitni stilovi prevazilaze tematsko stablo; kada je proslijeđen samo `styles`, on u potpunosti zamjenjuje podrazumijevano (originalno ponašanje, tako da postojeće integracije i skin‑ovi nisu pogođeni). `setupDarkModeSkin` je depreciran u korist `theme` propa.

### Configuration Options

Ova biblioteka nastoji da podrži sve konfiguracijske opcije definisane u [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), baš kao i web implementacija.

Pored toga, React Native dodaje nekoliko SDK‑specifičnih opcija putem `FastCommentsRNConfig`:

- `hideTopBar` – sakrij traku s prijavljenim korisnikom / zvonce za notifikacije iznad komponanta.
- `usePressToEdit` – pritisni‑i‑držati komentar da otvoriš njegov meni.
- `disableDownVoting` – sakrij dugmad za negativno glasanje.
- `renderCommentInline` – renderuj informacije o komentaru unutar istog HTML bloka kao sadržaj komentara.
- `renderLikesToRight` – pomjeri područje glasova/like‑ova desno od komentara umjesto ispod.
- `renderDateBelowComment` – prikazuj datum ispod komentara.
- `showLiveStatus` – prikaži chat‑stil “Live” + traku s brojem korisnika iznad komentara.
- `useInlineSubmitButton` – renderuj dugme za slanje kao ikonu unutar komponanta.
- `countAboveToggle` – uz `useShowCommentsToggle`, koliko komentara se renderuje iznad „Show Comments“ toggle‑a.
- `preserveFeedScrollPosition` – `FastCommentsFeed` pamti svoj scroll offset kroz unmount/remount (default true).

### FastComments Concepts

Glavni koncepti na koje treba obratiti pažnju za početak su `tenantId` i `urlId`. `tenantId` je identifikator vašeg FastComments.com naloga. `urlId` je entitet na koji će biti vezane niti komentarа. To može biti URL stranice, ID proizvoda, ID članka, itd.

### Localization

Sav tekst koji korisnici vide u ovim widgetima (oznake dugmadi, placeholderi, prazna stanja, relativni datumi poput “5 minutes ago”, poruke grešaka, itd.) je **server‑driven**. Komponente ne sadrže hard‑kodirane engleske stringove; renderuju prevode koje FastComments servira za traženi locale.

Da biste zatražili locale, postavite `locale` u vašoj konfiguraciji:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, itd.
};
```

Kada `locale` nije postavljen, FastComments servira podrazumijevani jezik tenant‑a.

**Uređivanje teksta:** prevodi se upravljaju u vašem FastComments kontrolnom panelu, ne u ovom SDK‑u. Da biste promijenili formulacije, prepišite podrazumijevani copy ili dodajte jezik, uredite prevode za vaš nalog u kontrolnom panelu – promjena se automatski primijeni na widgete bez potrebe za novim izdanjem aplikacije. SDK ne isporučuje engleske fallback‑e, pa bilo koji ključ koji ostavite prazan u kontrolnom panelu prikazuje se prazno; držite ključeve popunjenim za svaki locale koji podržavate.

### User Notifications

FastComments podržava notifikacije za [mnogo scenarija](https://docs.fastcomments.com/guide-notifications.html). Notifikacije su konfigurisane, mogu se globalno isključiti ili na nivou notifikacije/komentara, i podržavaju pretplate na nivou stranice tako da korisnici mogu pretplatiti niti specifične stranice ili članka.

Na primjer, moguće je koristiti Secure SSO za autentifikaciju korisnika, a zatim periodično poll‑ovati nepročitane notifikacije i push‑ovati ih korisniku.

Pogledajte [primjer AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za to kako dobiti i prevesti nepročitane notifikacije korisnika.

### Gif Browser

Po defaultu, niti odabir slike ni gif‑a nije omogućen. Pogledajte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/mainexample/src/AppCommentingImageSelection.tsx) za to kako podržati upload slika i gif‑ova. Postoji Gif Browser koji anonimizuje pretrage i slike pružene u ovoj biblioteci; vi ga samo trebate koristiti.

### Performance

Molimo otvorite tiket s primjerom za reprodukciju, uključujući uređaj koji ste koristili, ako identificirate bilo kakve probleme s performansama. Performanse su prioritetni aspekt svih FastComments biblioteka.