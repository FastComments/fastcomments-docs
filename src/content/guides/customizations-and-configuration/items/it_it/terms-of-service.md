FastComments consente di richiedere ai commentatori al primo commento di accettare i Termini di servizio (TOS) prima di inviare un commento.

Quando abilitato:
- **Utenti anonimi** visualizzeranno una casella di controllo dei Termini di servizio ogni volta che commentano
- **Utenti autenticati** visualizzeranno la casella solo al loro primo commento, o quando aggiorni i Termini di servizio

### Enabling Terms of Service

Vai alla pagina di personalizzazione del widget e abilita la casella "Richiedi accettazione dei Termini di servizio":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Customizing the TOS Text

Per impostazione predefinita, la casella visualizza "Accetto i Termini di servizio e l'Informativa sulla privacy" con link a entrambi i documenti. Puoi personalizzare questo testo per locale se necessario:

1. Seleziona "Personalizza testo per locale"
2. Seleziona la localizzazione dal menu a discesa e inserisci il tuo testo personalizzato

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Updating Your Terms of Service

Quando aggiorni i Termini di servizio, imposta la data "Ultimo aggiornamento". Gli utenti che hanno accettato i TOS prima di questa data saranno tenuti ad accettare nuovamente:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### How It Works

- La marca temporale di accettazione dei Termini di servizio viene memorizzata per utente e per commento
- Quando un utente accetta i TOS, la data viene registrata nel suo profilo utente (per-tenant)
- Se imposti una data "Ultimo aggiornamento" successiva alla data di accettazione dell'utente, dovranno accettare nuovamente
- Per gli utenti anonimi che non possono essere tracciati, la casella appare ad ogni invio di commento