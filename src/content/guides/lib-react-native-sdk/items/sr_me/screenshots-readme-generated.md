Live threaded commenting with avatars, nested replies, votes, and the built‑in rich‑text composer, plus a dark theme and a live‑chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Živo komantarisanje</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Živo komantarisanje, svijetla tema"/></td>
    <td align="center"><b>Tamna tema</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Živo komantarisanje, tamna tema"/></td>
    <td align="center"><b>Živi ćask</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Pretpostavljeni živi ćask"/></td>
  </tr>
</table>

### Rich Text Editor

Ova biblioteka koristi [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) za uređivanje rič‑teksta, što pruža moćno WYSIWYG iskustvo uređivanja. Isti editor pokreće iOS, Android i web (preko `react-native-web`), tako da se composer ponaša dosledno na svim platformama sa jedinstvenom implementacijom.

`react-native-enriched` zahteva React Native New Architecture (Fabric) na native (podrazumevano od RN 0.76, opcijski na RN 0.72‑0.75), i bundler koji rešava uslove `exports` paketa. Ovaj SDK je razvijan i testiran protiv RN 0.81 / React 19. Isti editor takođe radi na webu kroz `react-native-web`; web izgradnja enriched editora još uvek je označena kao eksperimentalna upstream.

### Widgets

SDK isporučuje tri vidžeta, koji odražavaju FastComments Android SDK:

- `FastCommentsLiveCommenting` – uvijeno (threaded) komentarisanje sa glasovima, odgovorima, paginacijom, spominjanjem, notifikacijama i ažuriranjima u realnom vremenu.
- `FastCommentsLiveChat` – pretpostavka za ćaskivanje nad istim motorom: hronološke poruke sa novim na dnu, composer ispod liste, traka naslova u realnom vremenu (tačka veze + broj korisnika), beskonačna istorija učitana skrolovanjem nagore, automatsko skrolovanje na nove poruke, bez glasova i uvijene strukture odgovora. Svaka pretpostavka može se nadjačati putem `config`.
- `FastCommentsFeed` – društveni feed sa composer‑om za postove, medijima, reakcijama, praćenjima i trakama za nove postove u realnom vremenu.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

Podrazumevani izgled generiše se iz skupa semantičkih dizajn tokena (`FastCommentsTheme`): boje, razmaci, radijusi, veličine fonta, debljine fonta i veličine avatara. Prosledite parcijalna prepisivanja tokena (tip `FastCommentsThemeOverrides`) kroz `theme` prop na bilo koji vidžet i cela stilizacijska stabla će se dosledno restilirati:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Tamni režim je samo jedan set tokena udaljen:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` prop i dalje prihvata sirovo `IFastCommentsStyles` stablo za preciznu kontrolu. Kada su `theme` i `styles` oba prosleđena, eksplicitni stilovi imaju prednost nad tematskim stablom; kada je prosleđen samo `styles`, on potpuno zamenjuje podrazumevane (originalno ponašanje, tako da postojeće integracije i teme nisu pogođene). `setupDarkModeSkin` je zastareo u korist `theme` prop‑a.

### Configuration Options

Ova biblioteka želi da podrži sve opcije konfiguracije definisane u [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), baš kao i web implementacija.

Pored njih, React Native dodaje nekoliko SDK‑specifičnih opcija kroz `FastCommentsRNConfig`:

- `hideTopBar` – sakrij traku korisnika/prijavljenog / zvono notifikacija iznad compose‑ra.
- `usePressToEdit` – pritisni i drži komentar da otvoriš njegov meni.
- `disableDownVoting` – sakrij dugmad za negativno glasanje.
- `renderCommentInline` – prikaži informacije o komentaru unutar istog HTML bloka kao i sadržaj komentara.
- `renderLikesToRight` – pomeri oblast glasanja/like‑ova desno od komentara umesto ispod.
- `renderDateBelowComment` – prikaži datum ispod komentara.
- `showLiveStatus` – prikaži traku naslova u stilu četa „Live“ + broj korisnika iznad komentara.
- `useInlineSubmitButton` – prikaži dugme za slanje kao ikonu unutar compose‑ra.
- `countAboveToggle` – uz `useShowCommentsToggle`, koliko komentara se prikazuje iznad prekidača „Show Comments“.
- `preserveFeedScrollPosition` – `FastCommentsFeed` pamti svoj pomak skrolovanja kroz unmount/remount (podrazumevano true).

### FastComments Concepts

Glavni koncepti koje treba razumjeti da biste počeli su `tenantId` i `urlId`. `tenantId` je identifikator vašeg FastComments.com naloga. `urlId` je entitet na koji će se veza thread‑ova komentara vezati. To može biti URL stranice, ID proizvoda, ID članka, itd.

### Localization

Sav tekst vidljiv korisniku u ovim vidžetima (natpisi dugmadi, placeholder‑i, prazna stanja, relativni datumi poput „5 minuta ago“, poruke o grešci, itd.) je **server‑driven**. Komponente ne sadrže hard‑kodirane engleske stringove; one renderuju prevode koje FastComments pruža za traženi locale.

Da biste zatražili locale, postavite `locale` u svojoj konfiguraciji:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, itd.
};
```

Kada `locale` nije postavljen, FastComments isporučuje podrazumevani jezik zakupca.

**Uređivanje teksta:** prevodi se upravljaju kroz vaš FastComments dashboard, ne kroz ovaj SDK. Da biste promenili formulaciju, nadjačajte podrazumevani tekst ili dodajte jezik, izmenite prevode za vaš nalog u dashboard‑u – promena se automatski prepoznaje od strane vidžeta bez potrebe za novim izdanjem aplikacije. SDK ne isporučuje engleske fallback‑e, pa svaki ključ koji ostavite praznim u dashboard‑u prikazuje se kao prazan; održavajte ključeve popunjene za svaki locale koji podržavate.

### User Notifications

FastComments podržava notifikacije za [mnogo scenarija](https://docs.fastcomments.com/guide-notifications.html). Notifikacije su konfigurabilne, mogu se globalno ili na nivou notifikacije/komentara isključiti, i podržavaju pretplate na nivou stranice tako da korisnici mogu pratiti thread‑ove određenih stranica ili članaka.

Na primer, moguće je koristiti Secure SSO za autentifikaciju korisnika i potom periodično proveravati nepročitane notifikacije i slati ih korisniku.

Pogledajte [primer AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za način kako dobiti i prevesti nepročitane korisničke notifikacije.

### Gif Browser

Podrazumevano, selekcija slika ili gif‑ova nije omogućena. Pogledajte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za način kako podržati otpremanje slika i gif‑ova. Biblioteka sadrži Gif Browser koji anonimno pretražuje i prikazuje slike; samo ga potrebno koristiti.

### Performance

Molimo otvorite tiket sa primerom koji reprodukuje problem, uključujući uređaj koji se koristi, ako identifikujete bilo kakve probleme sa performansama. Performanse su prioritetni aspekt svih FastComments biblioteka.