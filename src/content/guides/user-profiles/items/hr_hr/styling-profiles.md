Kada se korisnički profili otvore u kontekstu vaše stranice (putem widgeta za komentare), svi prilagođeni CSS stilovi koje ste primijenili na svoj FastComments widget automatski se ubacuju u modal profila.

### Kako to radi

Kada korisnik klikne na poveznicu profila iz vašeg widgeta za komentare, otvori se modal profila s klasom `.fast-comments-profile`. Prilagođeni CSS vašeg widgeta automatski se ubacuje u prikaz profila. Ako ste već stilizirali svoj widget za komentare, ti će se stilovi primijeniti i na profile.

### CSS klase

FastComments profili koriste arhitekturu CSS-a temeljenu na klasama. Ne koristi CSS custom properties.

Glavna stranica profila koristi `.user-profile` kao korijenski spremnik. Zaglavlje je `.profile-header` s `.profile-header-background` za pozadinsku sliku. Sadržaj profila nalazi se u `.profile-content`.

Avatar koristi `.profile-avatar` i `.profile-avatar-wrapper`. Ime korisnika je `.profile-name`, a tekst biografije je `.profile-bio`. Statistike su u `.profile-stats`, a pojedinačne statistike koriste `.stat`.

Društvene poveznice su u `.profile-social-links`, s pojedinačnim poveznicama kao `.social-link`. Značke koriste `.profile-badges` i `.badge`. Trake napretka za značke koriste `.progress-outer` i `.progress-bar`.

Kartice koriste `.profile-tabs` za spremnik, `.tab` za pojedinačne kartice i `.tab.active` za odabranu karticu. Sadržaj kartica koristi `.tab-body` i `.tab-body.active`. Brojke obavijesti na karticama koriste `.tab .count`.

Obavijesti koriste `.notification`, a DM razgovori koriste `.conversation`. Status online je `.activity-indicator` s `.activity-indicator.online` za aktivno stanje. Brojači nepročitanih koriste `.unread-count`.

Spremnik modala profila je `.fast-comments-profile` s `.fast-comments-profile-close` za gumb za zatvaranje.

### Tamni način

Tamni način koristi modifikator klase `.dark` na `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Primjeri

**Zaglavlje:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**Značke:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**Kartice:**
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