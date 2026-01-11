Wenn Benutzerprofile im Kontext Ihrer Site (über das Kommentar-Widget) geöffnet werden, werden alle benutzerdefinierten CSS-Stile, die Sie auf Ihr FastComments-Widget angewendet haben, automatisch in das Profil-Modal injiziert.

### Funktionsweise

Wenn ein Benutzer in Ihrem Kommentar-Widget auf einen Profil-Link klickt, öffnet sich ein Profil‑Modal mit der Klasse `.fast-comments-profile`. Die benutzerdefinierten CSS‑Stile Ihres Widgets werden automatisch in die Profilansicht injiziert. Wenn Sie Ihr Kommentar‑Widget bereits gestaltet haben, gelten diese Styles auch für Profile.

### CSS-Klassen

FastComments-Profile verwenden eine klassenbasierte CSS‑Architektur. Es werden keine CSS-Custom-Properties verwendet.

Die Hauptprofilseite verwendet `.user-profile` als Root-Container. Der Header-Bereich ist `.profile-header` mit `.profile-header-background` für das Hintergrundbild. Der Profilinhalt befindet sich in `.profile-content`.

Der Avatar verwendet `.profile-avatar` und `.profile-avatar-wrapper`. Der Benutzername ist `.profile-name` und der Biotext ist `.profile-bio`. Statistiken befinden sich in `.profile-stats`, einzelne Statistiken verwenden `.stat`.

Soziale Links befinden sich in `.profile-social-links`, einzelne Links sind `.social-link`. Abzeichen verwenden `.profile-badges` und `.badge`. Fortschrittsbalken für Abzeichen verwenden `.progress-outer` und `.progress-bar`.

Registerkarten verwenden `.profile-tabs` für den Container, `.tab` für einzelne Tabs und `.tab.active` für die ausgewählte Registerkarte. Der Inhalt der Registerkarte verwendet `.tab-body` und `.tab-body.active`. Benachrichtigungszähler auf Tabs verwenden `.tab .count`.

Benachrichtigungen verwenden `.notification` und Direktnachrichten-Unterhaltungen verwenden `.conversation`. Der Online-Status ist `.activity-indicator` mit `.activity-indicator.online` für den aktiven Zustand. Ungelesene Zähler verwenden `.unread-count`.

Der Profil-Modal-Container ist `.fast-comments-profile` mit `.fast-comments-profile-close` für die Schließen-Schaltfläche.

### Dunkelmodus

Der Dunkelmodus verwendet den Klassenmodifikator `.dark` auf `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Beispiele

**Kopfbereich:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**Abzeichen:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**Registerkarten:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**Modalfenster:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```