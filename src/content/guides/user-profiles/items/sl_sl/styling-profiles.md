Ko so uporabniški profili odprti v kontekstu vaše strani (prek pripomočka za komentarje), se vse prilagojene CSS-stilne spremembe, ki ste jih uporabili v vašem FastComments pripomočku, samodejno vstavijo v modalno okno profila.

### Kako deluje

Ko uporabnik klikne na povezavo profila iz vašega pripomočka za komentarje, se odpre modalno okno profila s class `.fast-comments-profile`. Prilagojeni CSS vašega pripomočka se samodejno vključi v pogled profila. Če ste že stilizirali svoj pripomoček za komentarje, se te stile uporabi tudi za profile.

### Razredi CSS

Profili FastComments uporabljajo arhitekturo CSS, osnovano na razredih. Ne uporabljajo CSS lastnosti po meri.

Glavna stran profila uporablja `.user-profile` kot korenski kontejner. Glavni del je `.profile-header` z `.profile-header-background` za ozadinsko sliko. Vsebina profila je v `.profile-content`.

Avatar uporablja `.profile-avatar` in `.profile-avatar-wrapper`. Ime uporabnika je `.profile-name` in besedilo bio je `.profile-bio`. Statistike so v `.profile-stats` z posameznimi statistikami, ki uporabljajo `.stat`.

Družabne povezave so v `.profile-social-links` s posameznimi povezavami kot `.social-link`. Značke uporabljajo `.profile-badges` in `.badge`. Napredek značk uporablja `.progress-outer` in `.progress-bar`.

Zavihki uporabljajo `.profile-tabs` za kontejner, `.tab` za posamezne zavihke in `.tab.active` za izbrani zavihek. Vsebina zavihkov uporablja `.tab-body` in `.tab-body.active`. Štetja obvestil na zavihkih uporabijo `.tab .count`.

Obvestila uporabljajo `.notification`, zasebna sporočila pa `.conversation`. Spletni status je `.activity-indicator` z `.activity-indicator.online` za aktiven status. Štetje neprebranih uporablja `.unread-count`.

Kontejner modalnega okna profila je `.fast-comments-profile` s `.fast-comments-profile-close` za gumb za zapiranje.

### Temni način

Temni način uporablja modifikator razreda `.dark` na `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Primeri

**Glava:**
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

**Zavihki:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**Modalno okno:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```