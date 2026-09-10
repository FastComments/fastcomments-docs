## Primer Zaps

Nekaj delovnih tokov, ki jih je mogoče nastaviti v nekaj minutah.

**Obvesti se o novih komentarjih.** Nov komentar, nato Slack "Send Channel Message" ali Discord "Send
Channel Message". Preslikaj ime komentatorja, besedilo komentarja in URL strani v sporočilo. Dodaj
filter domene, da obvestiš različni kanal za vsako spletno mesto.

**Vodi dnevnik vsakega komentarja.** Nov komentar, nato Google Sheets "Create Spreadsheet Row". Dodaj Izbrisan
komentar kot drugi Zap, ki doda vrstico z ID-jem komentarja, tako da list služi tudi kot revizijski sled.

**Pošlji e-pošto avtorju, ko je komentar odobren.** Posodobljen komentar z Zapier filtrom, kjer je Approved (odobreno) resničen,
nato Gmail "Send Email". Ker se Posodobljen komentar sproži ob vsaki spremembi, filter omogoča, da ta Zap
reagira le na odobritve.

**Dodaj komentatorje v svoj CRM ali poštni seznam.** Nov komentar, nato HubSpot "Create or Update Contact" ali
Mailchimp "Add or Update Subscriber" z uporabo e-pošte komentatorja. Spoštujte svojo politiko zasebnosti in lokalno zakonodajo
preden kdorkoli dodate na marketinški seznam.

**Ustvari komentar iz obrazca.** Typeform ali Google Forms "New Response", nato FastComments Create Comment
z ID-jem URL strani, ki ga vaše spletno mesto uporablja za pričevanja. Pustite polje Approved neoznačeno, da pregledate vsak komentar, preden se
prikaže.

**Objavi obvestila v vir.** RSS preko Zapier "New Item in Feed", nato Create Feed Post z elementa
naslov, vsebina in povezava.

**Omogoči članom SSO uporabnike.** Memberstack, Memberful ali vaš lastni webhook, nato Find SSO User, sledijo
Create SSO User v načinu "find or create" mode.

**Povečaj obravnavo prijavljenih komentarjev.** Posodobljen komentar, filtriran po številu zastav (flag) večjem od nič, nato Trello "Create
Card" ali Linear "Create Issue" z ID-jem komentarja in povezavo do strani za moderiranje.

**Objavi strani, ko postanejo žive.** WordPress ali Ghost "New Post", nato Create Page z URL-jem objave, tako da je
stran navedena in omejena pred prvim komentarjem.

**Arhiviraj izbrisane komentarje.** Izbrisan komentar, nato Airtable "Create Record" s celotnim komentarjem za
skladnost in hrambo.

---