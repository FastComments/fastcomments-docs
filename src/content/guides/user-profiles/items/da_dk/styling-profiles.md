Når brugerprofiler åbnes i konteksten af dit site (via kommentar-widgeten), indsættes eventuelle brugerdefinerede CSS-stilarter, du har anvendt på din FastComments-widget, automatisk i profilmodalen.

### Hvordan det virker

Når en bruger klikker på et profillink fra din kommentar-widget, åbnes en profilmodal med klassen `.fast-comments-profile`. Den brugerdefinerede CSS, du har anvendt på din widget, indsættes automatisk i profilvisningen. Hvis du allerede har stylet din kommentar-widget, vil disse stilarter også gælde for profiler.

### CSS-klasser

FastComments-profiler bruger en klassebaseret CSS-arkitektur. De bruger ikke CSS custom properties.

Den primære profilside bruger `.user-profile` som rodcontainer. Header-sektionen er `.profile-header` med `.profile-header-background` til baggrundsbilledet. Profilindholdet ligger i `.profile-content`.

Avataren bruger `.profile-avatar` og `.profile-avatar-wrapper`. Brugerens navn er `.profile-name` og biografteksten er `.profile-bio`. Statistikkerne findes i `.profile-stats` med individuelle statistikker ved brug af `.stat`.

Sociale links er i `.profile-social-links` med individuelle links som `.social-link`. Badges bruger `.profile-badges` og `.badge`. Badge-fremdriftsbjælker bruger `.progress-outer` og `.progress-bar`.

Faner bruger `.profile-tabs` som container, `.tab` til individuelle faner, og `.tab.active` for den valgte fane. Faneindhold bruger `.tab-body` og `.tab-body.active`. Notifikationstællinger på faner bruger `.tab .count`.

Notifikationer bruger `.notification` og private beskedsamtaler bruger `.conversation`. Online-status er `.activity-indicator` med `.activity-indicator.online` for den aktive tilstand. Ulæste tællere bruger `.unread-count`.

Profilmodalens container er `.fast-comments-profile` med `.fast-comments-profile-close` for lukknappen.

### Mørk tilstand

Mørk tilstand bruger klassemodifikatoren `.dark` på `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Eksempler

**Overskrift:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**Mærker:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**Faner:**
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