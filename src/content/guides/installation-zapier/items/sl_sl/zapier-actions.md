## Dejanja in Iskanja

Dejanja ustvarjajo podatke v FastComments; iskanja poiščejo podatke, da jih lahko kasnejši korak uporabi. Vsako dejanje kliče FastComments REST API in porabi enako število API kreditov, kot bi klic stal v vaši kodi: en kredit na klic, razen če je navedeno drugače.

## Ustvari komentar

Objavi komentar na strani.

| Polje | Obvezno | Opombe |
|-------|----------|-------|
| ID URL-ja strani | Da | ID URL-ja, ki ga pripomoček za komentarje uporablja na strani. Komentarji so po njem združeni. |
| URL strani | Da | Polni URL strani, ki se uporablja v e‑poštnih obvestilih. |
| Komentar | Da | Telo komentarja v FastComments markdownu. |
| Ime komentatorja | Da | Imena so edinstvena glede na e‑pošto, zato ponovna uporaba imena z drugo e‑pošto ne uspe. |
| E‑pošta komentatorja | Ne | Uporabnik se ustvari za e‑pošto, če še ne obstaja. |
| ID uporabnika | Ne | Obstoječi SSO ID uporabnika. Ima prednost pred imenom in e‑pošto. |
| ID nadrejenega komentarja | Ne | Nastavite za objavo odgovora. |
| Odobreno, Preverjeno | Ne | Oboje je privzeto nastavljeno na true. Neodobreni komentarji ostanejo skriti, dokler niso moderirani. |
| Objavljeno ob | Ne | Privzeto je trenutni čas. |
| URL avatarja, Naslov strani, Lokalizacija | Ne | Lokalizacija je privzeto `en_us`. |
| Prikaži v živo v pripomočku | Ne | Potiska komentar gledalcem v realnem času. Stane 2 kredita namesto 1. |
| Izvedi preverjanje spama, Pošlji e‑pošto | Ne | Privzeto izklopljeno. |

## Ustvari stran

Ustvari zapis strani, preden na njej obstaja kakšen komentar, tako da jo je mogoče izpisati in omejiti. Sprejme ID URL-ja, naslov, URL in po želji ID‑je SSO skupin, ki smejo stran videti.

## Ustvari SSO uporabnika

Ustvari uporabnika za enotno prijavo (single sign‑on). Sprejme vaš lastni ID uporabnika, uporabniško ime in e‑pošto, ter po želji prikazno ime, prikazno oznako, avatar, spletno stran, ID‑je skupin ter oznake za obvestila in zasebnost. Administrativnih vlog ni mogoče dodeliti iz Zapierja.

## Ustvari objavo v viru

Ustvari objavo v FastComments viru iz HTML vsebine. ID uporabnika avtorja je obvezen (FastComments ali SSO ID); naslov, oznake in en predogled povezave so neobvezni.

## Ustvari hashtag

Ustvari hashtag, ki ga lahko komentatorji uporabljajo, z neobveznim URL‑jem, na katerega se povezuje. Oznake so edinstvene za vsak račun, zato Zap, ki ustvarja eno pri vsakem zagonu, potrebuje nekaj edinstvenega v oznaki.

## Označi komentar

Označi komentar za pregled moderatorja. Zahtevan je ID uporabnika, ki označuje; ID avtorja, ki ga vrne Ustvari komentar, deluje.

## Iskanja

| Iskanje | Vnos | Vrne |
|--------|-------|------|
| Najdi komentar | ID komentarja | Komentar, ali nič. |
| Najdi SSO uporabnika | E‑pošta | SSO uporabnika, ali nič. |
| Najdi stran | ID URL-ja | Stran, ali nič. |

Iskanje, ki ne najde ničesar, ne povzroči napake v Zap‑u. Kombinirajte iskanje z ustvarjanjem v načinu "find or create" v Zapierju, da ustvarite stran ali uporabnika, ko manjka.