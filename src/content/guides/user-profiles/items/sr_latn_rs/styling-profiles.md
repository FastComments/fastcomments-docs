Kada se korisnički profili otvore u kontekstu vašeg sajta (putem widgeta za komentare), svi prilagođeni CSS stilovi koje ste primenili na vaš FastComments widget automatski se ubacuju u modal profila.

### Kako funkcioniše

Kada korisnik klikne na link profila iz vašeg komentarskog widgeta, otvori se modal profila sa klasom `.fast-comments-profile`. Prilagođeni CSS vašeg widgeta automatski se ubacuje u prikaz profila. Ako ste već stilizovali vaš komentarski widget, ti stilovi će se primenjivati i na profile.

### CSS klase

FastComments profili koriste arhitekturu CSS baziranu na klasama. Ne koriste CSS prilagođena svojstva.

Glavna stranica profila koristi `.user-profile` kao korenski kontejner. Zaglavlje sekcija je `.profile-header` sa `.profile-header-background` za pozadinsku sliku. Sadržaj profila se nalazi u `.profile-content`.

Avatar koristi `.profile-avatar` i `.profile-avatar-wrapper`. Ime korisnika je `.profile-name` a tekst bio je `.profile-bio`. Statistike su u `.profile-stats` pri čemu pojedinačne statistike koriste `.stat`.

Linkovi ka društvenim mrežama su u `.profile-social-links` sa pojedinačnim linkovima kao `.social-link`. Bedževi koriste `.profile-badges` i `.badge`. Progress bar-ovi bedževa koriste `.progress-outer` i `.progress-bar`.

Tabovi koriste `.profile-tabs` za kontejner, `.tab` za pojedinačne tabove, i `.tab.active` za izabrani tab. Sadržaj taba koristi `.tab-body` i `.tab-body.active`. Brojači notifikacija na tabovima koriste `.tab .count`.

Notifikacije koriste `.notification`, a DM konverzacije koriste `.conversation`. Online status je `.activity-indicator` sa `.activity-indicator.online` za aktivno stanje. Brojači nepročitanih koriste `.unread-count`.

Kontejner modal profila je `.fast-comments-profile` sa `.fast-comments-profile-close` za dugme za zatvaranje.

### Tamni režim

Tamni režim koristi modifikator klase `.dark` na `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Primeri

**Zaglavlje:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**Bedževi:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**Tabovi:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**Modalni prozor:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```