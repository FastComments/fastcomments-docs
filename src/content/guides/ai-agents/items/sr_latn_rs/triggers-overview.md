A **okidač** je događaj koji probudi agenta. Svaki agent može imati definisan jedan ili više okidača.

### Kompletna lista

| Trigger | When it fires |
|---|---|
| [Komentar dodat](#trigger-comment-add) | Novi komentar je objavljen. |
| [Komentar izmenjen](#trigger-comment-edit) | Komentar je izmenjen. Prethodni tekst je uključen u kontekst agenta. |
| [Komentar obrisan](#trigger-comment-delete) | Komentar je obrisan. |
| [Komentar prikvačen](#trigger-comment-pin) | Komentar je prikvačen (od bilo koga, uključujući moderatora ili drugog agenta). |
| [Komentar odknjižen](#trigger-comment-unpin) | Komentar je odknjižen. |
| [Komentar zaključan](#trigger-comment-lock) | Komentar je zaključan (nema više odgovora). |
| [Komentar otključan](#trigger-comment-unlock) | Komentar je otključan. |
| [Komentar prelazi prag glasova](#trigger-comment-vote-threshold) | Neto glasovi komentara dostižu konfigurisani prag. |
| [Komentar dostiže prag prijava](#trigger-comment-flag-threshold) | Broj prijava komentara tačno dostiže konfigurisani prag. |
| [Korisnik postavlja prvi komentar](#trigger-new-user-first-comment) | Korisnik postavlja svoj prvi komentar na ovom sajtu. |
| [Komentar automatski označen kao spam](#trigger-comment-auto-spammed) | Komentar je automatski označen kao spam od strane spam engine-a. |
| [Moderator pregleda komentar](#trigger-moderator-reviewed) | Moderator označava komentar kao pregledan. |
| [Moderator odobrava komentar](#trigger-moderator-approved) | Moderator odobrava komentar. |
| [Moderator označava kao spam](#trigger-moderator-spammed) | Moderator označava komentar kao spam. |
| [Moderator dodeljuje značku](#trigger-moderator-awarded-badge) | Moderator dodeljuje značku korisniku. |

### Više okidača po agentu

Agent može da se pretplati na bilo koju kombinaciju okidača - [Šablon moderatora](#template-moderator) se, na primer, pretplaćuje na oba `COMMENT_ADD` i `COMMENT_FLAG_THRESHOLD`. Svaki događaj pokreće agenta jednom sa kontekstom tog događaja.

### Šta sprečava pokretanje agenta

Pretplaćeni okidač događaja **ne** pokreće agenta ako važi bilo koja od sledećih stavki:

- Agentov [status](#status-states) je **Onemogućeno**.
- Agentov [URL ili opseg lokaliteta](#scope-url-locale) se ne poklapa sa komentarem koji je izazvao okidač.
- Agentov [dnevni, mesečni ili budžet po ograničenju stope](#budgets-overview) je iscrpljen - okidač je zabeležen kao **odbačen** sa razlogom. Vidi [Razlozi za odbacivanje](#drop-reasons).
- Konkurentnost za ovog agenta je zasićena (ograničeno po agentu).
- Tenant agenta ima nevažeću naplatu.
- Akciju koja je pokrenula okidač sam izvršio bot ili drugi agent (sprečavanje petlje).
- Okidač je bio za komentar koji je već obrađen od strane ovog agenta unutar prozora za deduplikaciju.

Kada se pretplaćeni okidač uspešno pokrene, agentova [Istorija pokretanja](#run-history) prikazuje red sa statusom **Pokrenuto** koji prelazi u **Uspešno** ili **Greška** kada se pokretanje završi.

### Pragovi glasova i prijava

Dva okidača - **Comment Crosses Vote Threshold** i **Comment Crosses Flag Threshold** - zahtevaju numerički prag na formi za izmenu. Okidač se aktivira u trenutku kada broj pređe konfigurisanu vrednost (konkretno, okidač za prag prijava se aktivira kada `flagCount === flagThreshold`, tako da izbor 1 znači "aktiviraj na prvoj prijavi", a izbor 5 znači "aktiviraj kada stigne peta prijava").

### Odloženi okidači

Bilo koji okidač može biti odložen tako da agent radi kasnije, na primer nakon što glasovi/prijave/odgovori imaju vremena da se stabilizuju. Vidi [Odloženi okidači](#trigger-deferred-delay).

### Sprečavanje petlji

Da bi se sprečile beskonačne petlje, komentari **koje piše agent** nose `botId`. Okidači za nove komentare ignorišu komentare sa `botId`.

Neto efekat: agenti mogu da deluju kao odgovor na *ljudske* akcije u vašem tenant-u, ali akcije koje potiču od agenata nikada ne pokreću nijedan agent-okidač. Ovo važi za sve tipove okidača.

### REPLAY: interni okidač

Postoji i interni tip okidača `REPLAY` koji koristi funkcija [Test pokretanja (Ponavljanja)](#test-runs-replays). Ne možete ga izabrati na formi za izmenu - postoji da bi se replay pokretanja jasno označila u istoriji pokretanja i isključila iz prikaza živih pokretanja.