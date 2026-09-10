## Primer Zapova

Nekoliko radnih tokova koji se postavljaju za par minuta.

**Budite obavešteni o novim komentarima.** Novi komentar, zatim Slack "Send Channel Message" ili Discord "Send Channel Message". Mapirajte ime komentatora, tekst komentara i URL stranice u poruku. Dodajte filter domena da obavestite različiti kanal po sajtu.

**Vodite evidenciju svakog komentara.** Novi komentar, zatim Google Sheets "Create Spreadsheet Row". Dodajte "Deleted Comment" kao drugi Zap koji dodaje red sa ID‑jem komentara, tako da tabela služi i kao revizijski zapis.

**Pošaljite e‑mail autoru kada je komentar odobren.** Ažurirani komentar sa Zapier filterom gde je Approved true, zatim Gmail "Send Email". Pošto se Ažurirani komentar aktivira pri svakoj promeni, filter je ono što ovaj Zap čini da reaguje samo na odobrenja.

**Dodajte komentatore u vaš CRM ili mailing listu.** Novi komentar, zatim HubSpot "Create or Update Contact" ili Mailchimp "Add or Update Subscriber" koristeći e‑mail komentatora. Poštujte vašu politiku privatnosti i lokalne zakone pre nego što nekoga dodate na marketinšku listu.

**Kreirajte komentar iz forme.** Typeform ili Google Forms "New Response", zatim FastComments Create Comment sa ID‑jem URL‑a stranice koji vaš sajt koristi za svedočanstva. Ostavite polje Approved neoznačeno da pregledate svaki komentar pre nego što se pojavi.

**Objavite najave u feed.** RSS by Zapier "New Item in Feed", zatim Create Feed Post sa naslovom, sadržajem i linkom stavke.

**Omogućite članove kao SSO korisnike.** Memberstack, Memberful ili vaš sopstveni webhook, zatim Find SSO User praćen sa Create SSO User u režimu "find or create".

**Eskalirajte prijavljene komentare.** Ažurirani komentar, filtriran na broj zastavica veći od nule, zatim Trello "Create Card" ili Linear "Create Issue" sa ID‑jem komentara i linkom do stranice za moderaciju.

**Objavite stranice kada postanu aktivne.** WordPress ili Ghost "New Post", zatim Create Page sa URL‑om posta, tako da je stranica navedena i ograničena pre prvog komentara.

**Arhivirajte izbrisane komentare.** Deleted Comment, zatim Airtable "Create Record" sa celim komentarom za čuvanje u skladu sa propisima.