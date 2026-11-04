## Primjeri Zapova

Nekoliko radnih tokova čije postavljanje traje samo nekoliko minuta.

**Dobijte obavijest o novim komentarima.** New Comment, a zatim Slack "Send Channel Message" ili Discord "Send Channel Message". Mapirajte ime komentatora, tekst komentara i URL stranice u poruku. Dodajte filter domene kako biste obavještavali različite kanale za različite stranice.

**Vodite evidenciju svakog komentara.** New Comment, a zatim Google Sheets "Create Spreadsheet Row". Dodajte Deleted Comment kao drugi Zap koji dodaje red s ID-om komentara, tako da tablica istovremeno služi kao revizijski trag.

**Pošaljite e-mail autoru kada je komentar odobren.** Updated Comment s Zapier filterom na Approved is true, a zatim Gmail "Send Email". Budući da se Updated Comment aktivira pri svakoj promjeni, upravo filter čini da ovaj Zap reagira samo na odobrenja.

**Dodajte komentatore u svoj CRM ili mailing listu.** New Comment, a zatim HubSpot "Create or Update Contact" ili Mailchimp "Add or Update Subscriber" koristeći e-mail komentatora. Poštujte svoju politiku privatnosti i lokalne zakone prije dodavanja ikoga na marketinšku listu.

**Stvorite komentar iz obrasca.** Typeform ili Google Forms "New Response", a zatim FastComments Create Comment s ID-om URL-a stranice koji vaša stranica koristi za preporuke. Ostavite Approved neoznačenim kako biste pregledali svaki komentar prije nego što se pojavi.

**Objavite najave u feed.** RSS by Zapier "New Item in Feed", a zatim Create Feed Post s naslovom, sadržajem i poveznicom stavke.

**Osigurajte članove kao SSO korisnike.** Memberstack, Memberful ili vlastiti webhook, a zatim Find SSO User nakon kojeg slijedi Create SSO User u načinu rada "find or create".

**Escalirajte prijavljene komentare.** Updated Comment, filtriran na broj zastavica veći od nule, a zatim Trello "Create Card" ili Linear "Create Issue" s ID-om komentara i poveznicom na stranicu za moderaciju.

**Objavite stranice čim postanu aktivne.** WordPress ili Ghost "New Post", a zatim Create Page s URL-om objave, tako da je stranica navedena i ograničena prije prvog komentara.

**Arhivirajte izbrisane komentare.** Deleted Comment, a zatim Airtable "Create Record" s potpunim komentarom radi usklađenosti s pravilima zadržavanja podataka.