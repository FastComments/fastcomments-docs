## Dejanja in Iskanja

Dejanja ustvarjajo podatke v FastComments; iskanja poizvedujejo podatke, da jih lahko kasnejši korak uporabi. Vsako dejanje kliče FastComments REST API in porabi enako število API kreditov, kot bi klic stal v vaši kodi: en kredit na klic, razen če je navedeno drugače.

## Ustvari komentar

Objavi komentar na strani.

| Polje | Obvezno | Opombe |
|-------|----------|-------|
| Page URL ID | Da | ID URL, ki ga pripomoček za komentarje uporablja na strani. Komentarji so po njem združeni. |
| Page URL | Da | Polni URL strani, ki se uporablja v e-poštnih obvestilih. |
| Comment | Da | Telo komentarja v FastComments markdownu. |
| Commenter Name | Da | Imena so edinstvena za vsako e-pošto, zato ponovna uporaba imena z drugo e-pošto ne uspe. |
| Commenter Email | Ne | Uporabnik se ustvari za e-pošto, če še ne obstaja. |
| User ID | Ne | Obstoječi SSO uporabniški ID. Ima prednost pred imenom in e-pošto. |
| Parent Comment ID | Ne | Nastavite za objavo odgovora. |
| Approved, Verified | Ne | Oboje je privzeto nastavljeno na true. Neodobreni komentarji ostanejo skriti, dokler niso moderirani. |
| Posted At | Ne | Privzeto je trenutni čas. |
| Avatar URL, Page Title, Locale | Ne | Jezik je privzeto `en_us`. |
| Show Live In Widget | Ne | Potiska komentar gledalcem v realnem času. Stane 2 kredita namesto 1. |
| Run Spam Check, Send Emails | Ne | Privzeto izklopljeno. |

## Ustvari stran

Ustvari zapis strani, preden na njej obstaja kakšen komentar, tako da jo je mogoče izpisati in omejiti. Sprejme ID URL, naslov, URL in po želji ID-je SSO skupin, ki smejo stran videti.

## Ustvari SSO uporabnika

Ustvari uporabnika za enotno prijavo (single sign‑on). Sprejme vaš lastni uporabniški ID, uporabniško ime in e‑pošto, ter po želji prikazno ime, prikazno oznako, avatar, spletno stran, ID‑je skupin ter zastavice za obvestila in zasebnost. Administrativnih vlog ni mogoče dodeliti prek Zapierja.

## Ustvari objavo v viru

Ustvari objavo v FastComments viru iz HTML vsebine, z izbirnim naslovom, avtorjem, oznakami in enim predogledom povezave.

## Ustvari hashtag

Ustvari hashtag, ki ga lahko komentatorji uporabljajo, z izbirnim URL‑jem, na katerega se poveže.

## Označi komentar

Označi komentar za pregled moderatorja. Podajte ID uporabnika, ki označuje, ali pustite prazno, da označite kot Zapier integracija.

## Iskanja

| Iskanje | Vhod | Vrne |
|--------|------|------|
| Najdi komentar | ID komentarja | Komentar ali nič. |
| Najdi SSO uporabnika | E‑pošta | SSO uporabnik ali nič. |
| Najdi stran | ID URL | Stran ali nič. |

Iskanje, ki ne najde ničesar, ne povzroči napake v Zap-u. Kombinirajte iskanje z ustvarjanjem v načinu „find or create“ v Zapierju, da ustvarite stran ali uporabnika, ko manjkata.