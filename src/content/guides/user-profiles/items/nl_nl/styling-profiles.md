---
Wanneer gebruikersprofielen worden geopend in de context van uw site (via de commentaarwidget), worden eventuele aangepaste CSS-stijlen die u op uw FastComments-widget hebt toegepast automatisch geïnjecteerd in de profielmodal.

### Hoe Het Werkt

Wanneer een gebruiker op een profielkoppeling in uw commentaarwidget klikt, opent een profielmodal met de klasse `.fast-comments-profile`. De aangepaste CSS van uw widget wordt automatisch geïnjecteerd in de profielweergave. Als u uw commentaarwidget al hebt gestyled, gelden die stijlen ook voor profielen.

### CSS-klassen

FastComments-profielen gebruiken een op klassen gebaseerde CSS-architectuur. Het gebruikt geen CSS custom properties.

De hoofdprofielpagina gebruikt `.user-profile` als hoofdcontainer. De headersectie is `.profile-header` met `.profile-header-background` voor de achtergrondafbeelding. De profielinhoud bevindt zich in `.profile-content`.

De avatar gebruikt `.profile-avatar` en `.profile-avatar-wrapper`. De naam van de gebruiker is `.profile-name` en de bio-tekst is `.profile-bio`. Statistieken bevinden zich in `.profile-stats` met individuele statistieken die `.stat` gebruiken.

Sociale links staan in `.profile-social-links` met individuele links als `.social-link`. Badges gebruiken `.profile-badges` en `.badge`. Voor voortgangsbalken van badges worden `.progress-outer` en `.progress-bar` gebruikt.

Tabs gebruiken `.profile-tabs` voor de container, `.tab` voor individuele tabs en `.tab.active` voor het geselecteerde tabblad. Inhoud van tabs gebruikt `.tab-body` en `.tab-body.active`. Aantal meldingen op tabs gebruikt `.tab .count`.

Notificaties gebruiken `.notification` en DM-gesprekken gebruiken `.conversation`. Online-status is `.activity-indicator` met `.activity-indicator.online` voor de actieve staat. Ongelezen tellers gebruiken `.unread-count`.

De container van de profielmodal is `.fast-comments-profile` met `.fast-comments-profile-close` voor de sluitknop.

### Donkere modus

Donkere modus gebruikt de klassemodifier `.dark` op `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Voorbeelden

**Header:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**Badges:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**Tabs:**
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

---