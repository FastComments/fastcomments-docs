Live grančasto komentiranje s avatarima, ugniježđenim odgovorima, glasovima i ugrađenim uređivačem bogatog teksta, plus tamna tema i predložak živog chata (prikazano ovdje renderirano putem `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Živo komentiranje</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Živo komentiranje, svijetla tema"/></td>
    <td align="center"><b>Tamna tema</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Živo komentiranje, tamna tema"/></td>
    <td align="center"><b>Živi chat</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Predložak živog chata"/></td>
  </tr>
</table>

### Uređivač bogatog teksta

Ova biblioteka koristi [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched-html) za uređivanje bogatog teksta, što pruža moćno WYSIWYG iskustvo uređivanja. Isti uređivač pogoni iOS, Android i web (preko `react-native-web`), pa se kompozitor ponaša dosljedno na svakoj platformi s jedinstvenom implementacijom.

`react-native-enriched` zahtijeva React Native New Architecture (Fabric) na native (zadano od RN 0.76, opcionalno na RN 0.72‑0.75), i bundler koji rješava `exports` uvjete paketa. Ovaj SDK je razvijen i testiran na RN 0.81 / React 19. Isti uređivač također radi na webu kroz `react-native-web`; web izgradnja enriched uređivača još uvijek je označena kao eksperimentalna uzvodno.

### Widgeti

SDK isporučuje tri widgeta, koji repliciraju FastComments Android SDK:

- `FastCommentsLiveCommenting` – grančasto komentiranje s glasovima, odgovorima, paginacijom, spominjanjima, obavijestima i živim ažuriranjima.
- `FastCommentsLiveChat` – predložak chata na istom motoru: kronološke poruke s novim na dnu, uređivač ispod popisa, traka zaglavlja uživo (točka veze + broj korisnika), beskonačna povijest učitana pomicanjem prema gore, automatsko pomicanje na nove poruke, bez glasova ili grananja odgovora. Svaki predložak može se nadjačati putem `config`.
- `FastCommentsFeed` – društveni feed s uređivačem objava, medijima, reakcijama, praćenjima i trakom za nove objave uživo.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Tematizacija

Zadani izgled generira se iz skupa semantičkih dizajnerskih tokena (`FastCommentsTheme`): boje, razmaci, radijusi, veličine fonta, debljine fonta i veličine avatara. Proslijedite parcijalna prepisivanja tokena (tipa `FastCommentsThemeOverrides`) kroz `theme` prop na bilo kojem widgetu i cijelo stablo stilova će se dosljedno preoblikovati:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

Tamni način je samo jedan set tokena udaljen:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

`styles` prop i dalje prihvaća sirovo `IFastCommentsStyles` stablo za kiruršku kontrolu. Kada su `theme` i `styles` oba pružena, eksplicitni stilovi imaju prednost nad tematskim stablom; kada je pružen samo `styles`, on u potpunosti zamjenjuje zadane (originalno ponašanje, pa postojeće integracije i skinovi nisu pogođeni). `setupDarkModeSkin` je zastario u korist `theme` prop-a.

### Opcije konfiguracije

Ova biblioteka nastoji podržati sve opcije konfiguracije definirane u [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), baš kao i web implementacija.

Uz to, React Native dodaje nekoliko SDK‑specifičnih opcija putem `FastCommentsRNConfig`:

- `hideTopBar` – sakrij traku s prijavljenim korisnikom / zvono obavijesti iznad uređivača.
- `usePressToEdit` – pritisni i drži komentar da otvoriš njegov izbornik.
- `disableDownVoting` – sakrij gumbe za negativno glasanje.
- `renderCommentInline` – prikaz informacija o komentatoru unutar istog HTML bloka kao sadržaj komentara.
- `renderLikesToRight` – premjesti područje glasova/likeova desno od komentara umjesto ispod.
- `renderDateBelowComment` – prikaz datuma ispod komentara.
- `showLiveStatus` – prikaži traku zaglavlja u stilu chata "Live" + broj korisnika iznad komentara.
- `useInlineSubmitButton` – prikaz gumba za slanje kao ikonu unutar uređivača.
- `countAboveToggle` – s `useShowCommentsToggle`, koliko komentara se prikazuje iznad preklopnika "Show Comments".
- `preserveFeedScrollPosition` – `FastCommentsFeed` pamti svoj pomak pomicanja kroz odmontiranje/ponovno montiranje (zadano true).

### FastComments koncepti

Glavni koncepti o kojima treba znati za početak su `tenantId` i `urlId`. `tenantId` je vaš identifikator računa na FastComments.com. `urlId` je mjesto na koje će se vezati niti komentara. To može biti URL stranice, ID proizvoda, ID članka, itd.

### Lokalizacija

Sav tekst koji korisnici vide u ovim widgetima (oznake gumba, placeholderi, prazna stanja, relativni datumi poput "5 minuta prije", poruke o greškama, itd.) je **održavan na poslužitelju**. Komponente ne kodiraju engleske stringove; one prikazuju prijevode koje FastComments pruža za traženi jezik.

Za zahtjev jezika, postavite `locale` u vašoj konfiguraciji:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, itd.
};
```

Kada `locale` nije postavljen, FastComments poslužuje zadani jezik najamnika.

**Uređivanje teksta:** prijevodi se upravljaju u vašoj FastComments nadzornoj ploči, ne u ovom SDK-u. Da biste promijenili formulacije, nadjačajte zadani tekst ili dodajte jezik, uredite prijevode za vaš račun u nadzornoj ploči – promjena se automatski primijeni na widgete bez potrebe za novim izdanjem aplikacije. SDK ne isporučuje engleske rezervne kopije, pa bilo koji ključ koji ostavite praznim u nadzornoj ploči prikazuje se prazno; držite ključeve popunjenima za svaki jezik koji podržavate.

### Obavijesti korisnika

FastComments podržava obavijesti za [mnogo scenarija](https://docs.fastcomments.com/guide-notifications.html). Obavijesti su konfigurabilne, mogu se globalno ili na razini obavijesti/komentara isključiti, i podržavaju pretplate na razini stranice kako bi korisnici mogli pretplatiti se na niti određene stranice ili članka.

Na primjer, moguće je koristiti Secure SSO za autentifikaciju korisnika, a zatim periodično provjeravati nepročitane obavijesti i slati ih korisniku.

Pogledajte [primjer AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) za način dobivanja i prevođenja nepročitanih obavijesti korisnika.

### Gif preglednik

Prema zadanim postavkama, odabir slike ili gif‑a nije omogućen. Pogledajte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) za način podrške učitavanja slika i gif‑ova. Postoji Gif preglednik koji anonimizira pretrage i slike pružene u ovoj biblioteci; jednostavno ga koristite.

### Performanse

Molimo otvorite tiket s primjerom za reprodukciju, uključujući korišteni uređaj, ako otkrijete bilo kakve probleme s performansama. Performanse su prioritetni aspekt svih FastComments biblioteka.