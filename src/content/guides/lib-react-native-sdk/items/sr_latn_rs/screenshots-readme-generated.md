Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live‑chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Živo komentarisanje</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Živo komentarisanje, svetla tema"/></td>
    <td align="center"><b>Tamna tema</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Živo komentarisanje, tamna tema"/></td>
    <td align="center"><b>Živi chat</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Živo chat podešavanje"/></td>
  </tr>
</table>

### Uređivač bogatog teksta

Ova biblioteka koristi [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) za uređivanje bogatog teksta, koji pruža moćno WYSIWYG iskustvo uređivanja. Ist isti uređivač napaja iOS, Android i web (preko `react-native-web`), tako da se kompozitor ponaša dosledno na svakoj platformi uz jedinstvenu implementaciju.

`react-native-enriched` zahteva React Native New Architecture (Fabric) na nativnom nivou (podrazumevano od RN 0.76, opcionalno od RN 0.72‑0.75), i bundler koji rešava uslove `exports` paketa. Ovaj SDK je razvijan i testiran na RN 0.81 / React 19. Ist isti uređivač radi i na webu kroz `react-native-web`; web‑build obogaćenog uređivača i dalje je označen kao eksperimentalni kod iznad.

### Vidžeti

SDK isporučuje tri vidžeta, koji su kopija FastComments Android SDK‑a:

- `FastCommentsLiveCommenting` – nitno komentarisanje sa glasovima, odgovorima, paginacijom, spominjanjima, obaveštenjima i ažuriranjima u realnom vremenu.
- `FastCommentsLiveChat` – chat preset preko istog motora: hronološke poruke sa novim na dnu, kompozitor ispod liste, traka zaglavlja u realnom vremenu (tačka veze + broj korisnika), beskonačna istorija učitana skrolovanjem nagore, automatsko skrolovanje na nove poruke, bez glasova ili odgovornih niti. Svaki preset može biti izmenjen putem `config`.
- `FastCommentsFeed` – društveni feed sa kompozitorom posta, medijima, reakcijama, praćenjima i banerima za nove postove u realnom vremenu.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Tematizacija

Podrazumevani izgled se generiše iz skupa semantičkih dizajn tokena (`FastCommentsTheme`): boje, razmaci, radijusi, veličine fonta, težine fonta i veličine avatara. Proslijedite parcijalna prepisivanja tokena (tipa `FastCommentsThemeOverrides`) preko `theme` prop‑a na bilo kojem vidžetu i ceo stil stabla se dosledno restajluje:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Tamni režim je jedan set tokena udaljen:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` prop i dalje prihvata sirovo `IFastCommentsStyles` stablo za preciznu kontrolu. Kada su `theme` i `styles` oba prosleđena, eksplicitni stilovi imaju prednost nad tematskim stablom; kada je prosleđen samo `styles`, on potpuno zamenjuje podrazumevane (originalno ponašanje, tako da postojeće integracije i skinovi nisu pogođeni). `setupDarkModeSkin` je zastareo u korist `theme` prop‑a.

### Opcije konfiguracije

Ova biblioteka želi da podrži sve opcije konfiguracije definisane u [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), baš kao i web implementacija.

Pored toga, React Native dodaje nekoliko specifičnih SDK opcija putem `FastCommentsRNConfig`:

- `hideTopBar` – sakrij traku sa prijavljenim korisnikom / zvonceom za obaveštenja iznad kompozitora.
- `usePressToEdit` – pritisni i drži komentar da otvoriš njegov meni.
- `disableDownVoting` – sakrij dugmad za negativno glasanje.
- `renderCommentInline` – prikaži informacije o komentatoru unutar istog HTML bloka kao sadržaj komentara.
- `renderLikesToRight` – pomeri oblast glasova/like‑ova desno od komentara umesto ispod njega.
- `renderDateBelowComment` – prikaži datum ispod komentara.
- `showLiveStatus` – prikaži traku zaglavlja u stilu četa “Live” + broj korisnika iznad komentara.
- `useInlineSubmitButton` – prikaži dugme za slanje kao ikonu unutar kompozitora.
- `countAboveToggle` – sa `useShowCommentsToggle`, koliko komentara se prikazuje iznad prekidača “Show Comments”.
- `preserveFeedScrollPosition` – `FastCommentsFeed` pamti svoj pomak skrolovanja kroz deinstalacije/ponovna učitavanja (podrazumevano true).

### FastComments koncepti

Glavni koncepti o kojima treba da znate da biste počeli su `tenantId` i `urlId`. `tenantId` je identifikator vašeg FastComments.com naloga. `urlId` je mesto na koje će biti vezane niti komentara. Ovo može biti URL stranice, ID proizvoda, ID članka, itd.

### Lokalizacija

Sav tekst koji vidi korisnik u ovim vidžetima (oznake dugmadi, placeholder‑i, prazna stanja, relativni datumi poput “5 minuta ago”, poruke o greškama, itd.) je **server‑driven**. Komponente ne hard‑kodiraju engleske stringove; one renderuju prevode koje FastComments pruža za traženi locale.

Da biste zatražili locale, postavite `locale` u vašoj konfiguraciji:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, itd.
};
```

Kada `locale` nije postavljen, FastComments pruža podrazumevani jezik tenant‑a.

**Uređivanje teksta:** prevodi se upravljaju u vašem FastComments kontrolnoj tabli, a ne u ovom SDK‑u. Da biste promenili formulaciju, prepišite podrazumevani tekst ili dodajte jezik, izmenite prevode za vaš nalog u kontrolnoj tabli – promena se automatski primeni na vidžete bez potrebe za novim izdavanjem aplikacije. SDK ne isporučuje engleske fallback‑e, tako da bilo koji ključ koji ostavite prazan u kontrolnoj tabli prikazuje se kao prazno; držite ključeve popunjenim za svaki locale koji podržavate.

### Obaveštenja korisnika

FastComments podržava obaveštenja za [mnogo scenarija](https://docs.fastcomments.com/guide-notifications.html). Obaveštenja su konfigurisana, mogu se globalno isključiti ili na nivou obaveštenja/komentara, i podržavaju pretplate na nivou stranice kako bi korisnici mogli da se pretplate na niti određene stranice ili članka.

Na primer, moguće je koristiti Secure SSO da autentifikujete korisnika, a zatim periodično proveravati nepročitana obaveštenja i slati ih korisniku.

Pogledajte [primer AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za način dobijanja i prevođenja nepročitanih obaveštenja korisnika.

### Gif pregledač

Podrazumevano, nijedan odabir slike ili gif‑a nije omogućen. Pogledajte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/mainexample/src/AppCommentingImageSelection.tsx) za način podrške za učitavanje slika i gif‑ova. Postoji Gif pregledač koji anonimno pretražuje i slike pružene u ovoj biblioteci; jednostavno ga koristite.

### Performans

Molimo otvorite tiket sa primerom za reprodukciju, uključujući uređaj koji se koristi, ako identifikujete bilo koje probleme sa performansama. Performans je prvi‑klasni građanin svih FastComments biblioteka.