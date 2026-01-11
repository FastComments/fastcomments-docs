Kada se korisnički profili otvore u kontekstu vaše stranice (putem widgeta za komentare), svi prilagođeni CSS stilovi koje ste primijenili na vaš FastComments widget automatski se ubacuju u modal profila.

### Kako to funkcioniše

Kada korisnik klikne na link profila iz vašeg widgeta za komentare, otvori se modal profila sa klasom `.fast-comments-profile`. Prilagođeni CSS vašeg widgeta automatski se ubacuje u prikaz profila. Ako ste već stilizovali vaš widget za komentare, ti stilovi će se primjenjivati i na profile.

### CSS klase

FastComments profili koriste arhitekturu CSS zasnovanu na klasama. Ne koriste CSS prilagođena svojstva.

Glavna stranica profila koristi `.user-profile` kao korenski kontejner. Sekcija zaglavlja je `.profile-header` sa `.profile-header-background` za pozadinsku sliku. Sadržaj profila nalazi se u `.profile-content`.

Avatar koristi `.profile-avatar` i `.profile-avatar-wrapper`. Ime korisnika je `.profile-name`, a tekst bio je `.profile-bio`. Statistike se nalaze u `.profile-stats` dok pojedinačne statistike koriste `.stat`.

Društvene veze su u `.profile-social-links`, pojedinačne veze su `.social-link`. Značke koriste `.profile-badges` i `.badge`. Trake napretka znački koriste `.progress-outer` i `.progress-bar`.

Kartice koriste `.profile-tabs` za kontejner, `.tab` za pojedinačne kartice, i `.tab.active` za odabranu karticu. Sadržaj kartica koristi `.tab-body` i `.tab-body.active`. Brojači obavijesti na karticama koriste `.tab .count`.

Obavijesti koriste `.notification`, a privatne poruke koriste `.conversation`. Status online je `.activity-indicator` sa `.activity-indicator.online` za aktivno stanje. Brojači nepročitanih koriste `.unread-count`.

Kontejner modala profila je `.fast-comments-profile` sa `.fast-comments-profile-close` za dugme za zatvaranje.

### Tamni režim

Dark mode uses the `.dark` class modifier on `.user-profile`.

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

**Modal:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```