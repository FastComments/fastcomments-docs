Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Živo komentarisanje</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Živo komentarisanje, svetla tema"/></td>
    <td align="center"><b>Tamna tema</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Živo komentarisanje, tamna tema"/></td>
    <td align="center"><b>Živi chat</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Preset za živi chat"/></td>
  </tr>
</table>

### Rich Text Editor

Ova biblioteka koristi [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) za uređivanje rich teksta, što pruža moćno WYSIWYG iskustvo uređivanja. Isti editor pokreće iOS, Android i web (preko `react-native-web`), tako da se sastavljač ponaša dosledno na svim platformama uz jedinstvenu implementaciju.

`react-native-enriched` zahteva React Native New Architecture (Fabric) na native (podrazumevano od RN 0.76, opcionalno na RN 0.72‑0.75), i bundler koji razrešava uslove `exports` paketa. Ovaj SDK je razvijen i testiran protiv RN 0.81 / React 19. Isti editor takođe radi na webu preko `react-native-web`; web izgradnja enriched editora i dalje je označena kao eksperimentalna uz upstream.

### Vidžeti

SDK isporučuje tri vidžeta, koji odražavaju FastComments Android SDK:

- `FastCommentsLiveCommenting` - komentarisanje u nitima sa glasovima, odgovorima, paginacijom, spominjanjima, notifikacijama i živim ažuriranjima.
- `FastCommentsLiveChat` - preset za chat preko istog motora: hronološke poruke sa novim na dnu, sastavljač ispod liste, živa traka zaglavlja (tačka veze + broj korisnika), beskonačna istorija učitana skrolovanjem nagore, automatsko skrolovanje do novih poruka, bez glasova ili ugnježdenih odgovora. Svaki preset može biti prepisan putem `config`.
- `FastCommentsFeed` - društveni feed sa sastavljačem postova, medijima, reakcijama, praćenjima i živim banerima za nove postove.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Temiranje

Podrazumevani izgled se generiše iz skupa semantičkih dizajn tokena (`FastCommentsTheme`): boje, razmaci, radijusi, veličine fontova, debljine fontova i veličine avatara. Proslijedite parcijalna prepisivanja tokena (tipa `FastCommentsThemeOverrides`) kroz `theme` prop na bilo kojem vidžetu i čitavo stablo stilova će se dosledno preoblikovati:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Tamni režim je jedan set tokena udaljen:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` prop i dalje prihvata sirovo `IFastCommentsStyles` stablo za preciznu kontrolu. Kada su `theme` i `styles` oba prosleđena, eksplicitni stilovi imaju prednost nad temiranim stablom; kada je prosleđen samo `styles`, on potpuno zamenjuje podrazumevane (originalno ponašanje, tako da postojeće integracije i skinovi nisu pogođeni). `setupDarkModeSkin` je zastareo u korist `theme` prop-a.

### Opcije konfiguracije

Ova biblioteka ima za cilj da podrži sve opcije konfiguracije definisane u [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), baš kao i web implementacija.

Pored toga, React Native dodaje nekoliko SDK‑specifičnih opcija putem `FastCommentsRNConfig`:

- `hideTopBar` - sakriva traku sa prijavljenim korisnikom / zvonce za notifikacije iznad sastavljača.
- `usePressToEdit` - pritisni i drži komentar da otvoriš njegov meni.
- `disableDownVoting` - sakriva dugmad za negativno glasanje.
- `renderCommentInline` - renderuje informacije o komentaru unutar istog HTML bloka kao sadržaj komentara.
- `renderLikesToRight` - pomera oblast glasova/like-ova desno od komentara umesto ispod njega.
- `renderDateBelowComment` - prikazuje datum ispod komentara.
- `showLiveStatus` - prikazuje traku zaglavlja u stilu četa "Live" + broj korisnika iznad komentara.
- `useInlineSubmitButton` - renderuje dugme za slanje kao ikonu unutar sastavljača.
- `countAboveToggle` - zajedno sa `useShowCommentsToggle`, koliko komentara se renderuje iznad prekidača "Show Comments".
- `preserveFeedScrollPosition` - `FastCommentsFeed` pamti svoj pomeraj skrolovanja kroz unmount/remount (podrazumevano true).

### FastComments koncepti

Glavni koncepti o kojima treba da budete svesni da biste započeli su `tenantId` i `urlId`. `tenantId` je identifikator vašeg FastComments.com naloga. `urlId` je mesto na koje će se vezivati niti komentara. To može biti URL stranice, ID proizvoda, ID članka, itd.

### Lokalizacija

Sav tekst koji vidi korisnik u ovim vidžetima (oznake dugmadi, placeholderi, prazna stanja, relativni datumi poput "5 minuta ago", poruke o greškama, itd.) je **server‑driven**. Komponente ne sadrže hard‑kodirane engleske stringove; one renderuju prevode koje FastComments pruža za traženi jezik.

Da biste zatražili jezik, postavite `locale` u vašoj konfiguraciji:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Kada `locale` nije postavljen, FastComments pruža podrazumevani jezik zakupca.

**Uređivanje teksta:** prevodi se upravljaju u vašem FastComments kontrolnom panelu, a ne u ovom SDK‑u. Da biste promenili formulaciju, prepišite podrazumevani tekst ili dodajte jezik, izmenite prevode za vaš nalog u kontrolnom panelu – promena se automatski preuzima u vidžete bez potrebe za novim izdanjem aplikacije. SDK ne isporučuje engleske rezervne kopije, tako da bilo koji ključ koji ostavite praznim u kontrolnom panelu renderuje prazno; držite ključeve popunjenim za svaki jezik koji podržavate.

### Notifikacije korisnika

FastComments podržava notifikacije za [mnoge scenarije](https://docs.fastcomments.com/guide-notifications.html). Notifikacije su konfigurisane, mogu se globalno isključiti ili na nivou notifikacije/komentara, i podržavaju pretplate na nivou stranice tako da korisnici mogu da se pretplate na niti određene stranice ili članka.

Na primer, moguće je koristiti Secure SSO za autentifikaciju korisnika, a zatim periodično proveravati nepročitane notifikacije i slati ih korisniku.

Pogledajte [primer AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za način dobijanja i prevođenja nepročitanih notifikacija korisnika.

### Gif pregledač

Podrazumevano, nijedan izbor slika ili gif‑ova nije omogućen. Pogledajte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za način podrške otpremanja slika i gif‑ova. Postoji Gif pregledač koji anonimno pretražuje i pruža slike u ovoj biblioteci, jednostavno ga koristite.

### Performans

Molimo otvorite tiket sa primerom za reprodukciju, uključujući korišćeni uređaj, ako identifikujete bilo kakve probleme sa performansama. Performans je prioritet u svim FastComments bibliotekama.