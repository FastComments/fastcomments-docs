## Przykładowe Zapy

Kilka workflow, których skonfigurowanie zajmuje kilka minut.

**Otrzymuj powiadomienia o nowych komentarzach.** New Comment, a następnie Slack "Send Channel Message" lub Discord "Send Channel Message". Zmapuj nazwę komentującego, treść komentarza i adres URL strony do wiadomości. Dodaj filtr domeny, aby powiadamiać inny kanał dla każdej witryny.

**Prowadź dziennik każdego komentarza.** New Comment, a następnie Google Sheets "Create Spreadsheet Row". Dodaj Deleted Comment jako drugiego Zapa, który dopisuje wiersz z identyfikatorem komentarza, dzięki czemu arkusz pełni jednocześnie rolę dziennika audytu.

**Wysyłaj e-mail do autora po zatwierdzeniu komentarza.** Updated Comment z filtrem Zapier na Approved is true, a następnie Gmail "Send Email". Ponieważ Updated Comment uruchamia się przy każdej zmianie, to filtr sprawia, że Zap reaguje tylko na zatwierdzenia.

**Dodawaj komentujących do swojego CRM lub listy mailingowej.** New Comment, a następnie HubSpot "Create or Update Contact" lub Mailchimp "Add or Update Subscriber" z użyciem adresu e-mail komentującego. Przed dodaniem kogokolwiek do listy marketingowej przestrzegaj swojej polityki prywatności i lokalnego prawa.

**Twórz komentarze z formularza.** Typeform lub Google Forms "New Response", a następnie FastComments Create Comment z identyfikatorem URL strony, którego Twoja witryna używa dla opinii. Pozostaw Approved niezaznaczone, aby przejrzeć każdy komentarz przed jego pojawieniem się.

**Publikuj ogłoszenia w kanale.** RSS by Zapier "New Item in Feed", a następnie Create Feed Post z tytułem, treścią i linkiem elementu.

**Zapewnij członkom konta SSO.** Memberstack, Memberful lub własny webhook, a następnie Find SSO User, po którym następuje Create SSO User w trybie "find or create".

**Eskaluj zgłoszone komentarze.** Updated Comment, filtrowany po liczbie flag większej od zera, a następnie Trello "Create Card" lub Linear "Create Issue" z identyfikatorem komentarza i linkiem do strony moderacji.

**Publikuj strony, gdy tylko stają się aktywne.** WordPress lub Ghost "New Post", a następnie Create Page z adresem URL wpisu, dzięki czemu strona jest widoczna na liście i ograniczona przed pierwszym komentarzem.

**Archiwizuj usunięte komentarze.** Deleted Comment, a następnie Airtable "Create Record" z pełną treścią komentarza dla celów zgodności i retencji.