FastComments ti permette di richiedere ai commentatori al primo commento di accettare i tuoi Termini di servizio prima di inviare un commento.

When enabled:
- **Anonymous users** will see a TOS checkbox every time they comment
- **Authenticated users** will see the checkbox only on their first comment, or when you update your TOS

### Configurazione

Navigate to the widget customization page and enable the "Require Terms of Service acceptance" checkbox. Once enabled, you'll see the following options:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **TOS Text Mode**: Per impostazione predefinita, la casella mostra "Accetto i Termini di servizio e l'Informativa sulla privacy" con collegamenti a entrambi i documenti. Seleziona "Personalizza il testo per locale" per fornire il tuo testo per ogni lingua.
- **TOS Last Updated Date**: Quando aggiorni i tuoi Termini di servizio, imposta questa data. Gli utenti che hanno accettato prima di questa data saranno obbligati ad accettare nuovamente.

### Come funziona

- Il timestamp di accettazione dei TOS viene memorizzato per utente e per commento
- Quando un utente accetta i TOS, la data viene registrata nel suo profilo utente (per-tenant)
- Se imposti una data "Ultimo aggiornamento" che è successiva alla data di accettazione dell'utente, dovranno accettare nuovamente
- Per gli utenti anonimi che non possono essere tracciati, la casella appare ad ogni invio di commento