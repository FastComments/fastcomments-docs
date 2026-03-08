I ruoli di Canvas vengono mappati automaticamente sui ruoli di FastComments durante il lancio LTI. Non è necessaria alcuna configurazione manuale.

#### Mappatura dei ruoli

| Canvas Role | FastComments Role | Permissions |
|---|---|---|
| **Amministratore** | Admin | Accesso completo all'account, gestione di tutti i commenti e delle impostazioni |
| **Docente** | Moderator | Modificare ed eliminare commenti, fissare i thread, gestire le discussioni |
| **Studente** | Commenter | Pubblicare commenti, rispondere, votare e usare le menzioni |

#### Come funziona

Quando un utente avvia FastComments da Canvas, il protocollo LTI 1.3 include il suo ruolo Canvas. FastComments legge questo ruolo e assegna automaticamente le autorizzazioni appropriate.

Se un utente ha più ruoli (ad es. un Docente che è anche Admin), viene utilizzato il ruolo con il privilegio più elevato.

---