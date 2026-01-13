---
Quando i Profili utente vengono aperti nel contesto del tuo sito (tramite il widget dei commenti), eventuali stili CSS personalizzati che hai applicato al tuo widget FastComments vengono iniettati automaticamente nella finestra modale del profilo.

### Come Funziona

Quando un utente clicca su un link del profilo dal tuo widget dei commenti, si apre una finestra modale del profilo con la classe `.fast-comments-profile`. Il CSS personalizzato del tuo widget viene automaticamente iniettato nella vista del profilo. Se hai già stilizzato il tuo widget dei commenti, quegli stili si applicheranno anche ai profili.

### Classi CSS

I profili FastComments utilizzano un'architettura CSS basata su classi. Non utilizza proprietà CSS personalizzate.

La pagina principale del profilo utilizza `.user-profile` come contenitore radice. La sezione header è `.profile-header` con `.profile-header-background` per l'immagine di sfondo. Il contenuto del profilo si trova in `.profile-content`.

L'avatar usa `.profile-avatar` e `.profile-avatar-wrapper`. Il nome dell'utente è `.profile-name` e il testo della biografia è `.profile-bio`. Le statistiche sono in `.profile-stats` con singole statistiche che usano `.stat`.

I link social si trovano in `.profile-social-links` con singoli link come `.social-link`. I badge usano `.profile-badges` e `.badge`. Le barre di progresso dei badge usano `.progress-outer` e `.progress-bar`.

Le schede usano `.profile-tabs` per il contenitore, `.tab` per le schede individuali, e `.tab.active` per la scheda selezionata. Il contenuto delle schede usa `.tab-body` e `.tab-body.active`. I conteggi delle notifiche sulle schede usano `.tab .count`.

Le notifiche usano `.notification` e le conversazioni DM usano `.conversation`. Lo stato online è `.activity-indicator` con `.activity-indicator.online` per lo stato attivo. I contatori di elementi non letti usano `.unread-count`.

Il contenitore della finestra modale del profilo è `.fast-comments-profile` con `.fast-comments-profile-close` per il pulsante di chiusura.

### Modalità Scura

La modalità scura utilizza il modificatore di classe `.dark` su `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Esempi

**Intestazione:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**Badge:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**Schede:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**Modale:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```

---