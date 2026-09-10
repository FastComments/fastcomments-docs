## Primjeri Zapova

Nekoliko radnih tokova koji se postavljaju za par minuta.

**Primajte obavijesti o novim komentarima.** New Comment, then Slack "Send Channel Message" or Discord "Send
Channel Message". Mapirajte ime komentatora, tekst komentara i URL stranice u poruku. Dodajte
filter domene kako biste obavijestili različiti kanal po web mjestu.

**Vodite zapis svakog komentara.** New Comment, then Google Sheets "Create Spreadsheet Row". Dodajte Deleted
Comment kao drugi Zap koji dodaje redak s ID‑om komentara, tako da tablica služi i kao revizijski zapis.

**Pošaljite e‑mail autoru kada je komentar odobren.** Updated Comment with a Zapier filter on Approved is true,
then Gmail "Send Email". Budući da Updated Comment aktivira pri svakoj promjeni, filter je ono što
čini da ovaj Zap reagira samo na odobrenja.

**Dodajte komentatore u svoj CRM ili mailing listu.** New Comment, then HubSpot "Create or Update Contact" or
Mailchimp "Add or Update Subscriber" using the commenter email. Poštujte svoju politiku privatnosti i lokalne zakone
prije nego što ikoga dodate na marketinšku listu.

**Stvorite komentar iz obrasca.** Typeform or Google Forms "New Response", then FastComments Create Comment
with the page URL ID your site uses for testimonials. Ostavite Approved neoznačeno kako biste pregledali svaki prije nego što se
prikaže.

**Objavite najave u feed.** RSS by Zapier "New Item in Feed", then Create Feed Post with the item's
title, content, and link.

**Omogućite članove kao SSO korisnike.** Memberstack, Memberful, or your own webhook, then Find SSO User followed
by Create SSO User in "find or create" mode.

**Eskalirajte prijavljene komentare.** Updated Comment, filtered on a flag count above zero, then Trello "Create
Card" or Linear "Create Issue" with the comment id and a link to the moderation page.

**Objavite stranice kada postanu aktivne.** WordPress or Ghost "New Post", then Create Page with the post URL, so the
page is listed and restricted before the first comment.

**Arhivirajte izbrisane komentare.** Deleted Comment, then Airtable "Create Record" with the full comment for
compliance retention.