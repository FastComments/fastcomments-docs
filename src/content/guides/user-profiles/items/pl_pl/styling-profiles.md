Kiedy profile użytkowników są otwierane w kontekście Twojej witryny (poprzez widget komentarzy), wszystkie niestandardowe style CSS, które zastosowałeś w swoim widżecie FastComments, są automatycznie wstrzykiwane do modala profilu.

### Jak to działa

Kiedy użytkownik kliknie link do profilu wewnątrz Twojego widgetu komentarzy, otwiera się modal profilu z klasą `.fast-comments-profile`. Niestandardowe style CSS Twojego widgetu są automatycznie wstrzykiwane do widoku profilu. Jeśli już ostylowałeś swój widget komentarzy, te style zostaną zastosowane także do profili.

### Klasy CSS

Profile FastComments używają architektury CSS opartej na klasach. Nie korzystają z niestandardowych właściwości CSS.

Główna strona profilu używa `.user-profile` jako kontenera głównego. Sekcja nagłówka to `.profile-header` z `.profile-header-background` dla obrazu tła. Zawartość profilu znajduje się w `.profile-content`.

Awatar używa `.profile-avatar` i `.profile-avatar-wrapper`. Nazwa użytkownika to `.profile-name`, a tekst bio to `.profile-bio`. Statystyki znajdują się w `.profile-stats`, a poszczególne statystyki używają `.stat`.

Linki społecznościowe znajdują się w `.profile-social-links`, a poszczególne linki mają klasę `.social-link`. Odznaki korzystają z `.profile-badges` i `.badge`. Paski postępu odznak używają `.progress-outer` i `.progress-bar`.

Karty używają `.profile-tabs` jako kontenera, `.tab` dla poszczególnych kart oraz `.tab.active` dla wybranej karty. Zawartość kart używa `.tab-body` i `.tab-body.active`. Liczniki powiadomień na kartach używają `.tab .count`.

Powiadomienia używają `.notification`, a konwersacje prywatne (DM) używają `.conversation`. Status online to `.activity-indicator` z `.activity-indicator.online` dla stanu aktywnego. Liczniki nieprzeczytanych używają `.unread-count`.

Kontener modala profilu to `.fast-comments-profile`, z `.fast-comments-profile-close` dla przycisku zamykania.

### Tryb ciemny

Tryb ciemny używa modyfikatora klasy `.dark` na `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Przykłady

**Nagłówek:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**Odznaki:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**Karty:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**Okno modalne:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```

---