A **okidač** je događaj koji probudi agenta. Svaki agent može imati definirano jedan ili više okidača.

### Potpun popis

| Okidač | Kada se aktivira |
|---|---|
| [Komentar dodan](#trigger-comment-add) | Objavljen je novi komentar. |
| [Komentar uređen](#trigger-comment-edit) | Komentar je uređen. Prethodni tekst je uključen u kontekst agenta. |
| [Komentar izbrisan](#trigger-comment-delete) | Komentar je izbrisan. |
| [Komentar prikvačen](#trigger-comment-pin) | Komentar je prikvačen (od strane bilo koga, uključujući moderatora ili drugog agenta). |
| [Komentar uklonjen s prikvačenja](#trigger-comment-unpin) | Komentar je uklonjen s prikvačenja. |
| [Komentar zaključan](#trigger-comment-lock) | Komentar je zaključan (nije dopušteno daljnje odgovaranje). |
| [Komentar otključan](#trigger-comment-unlock) | Komentar je otključan. |
| [Komentar prelazi prag glasova](#trigger-comment-vote-threshold) | Net glasovi komentara dosegnu konfigurirani prag. |
| [Komentar prelazi prag prijava](#trigger-comment-flag-threshold) | Broj prijava komentara točno doseže konfigurirani prag. |
| [Korisnik objavljuje prvi komentar](#trigger-new-user-first-comment) | Korisnik objavljuje svoj prvi komentar na ovom web-mjestu. |
| [Komentar automatski označen kao spam](#trigger-comment-auto-spammed) | Komentar se automatski označi kao spam od strane spam-mehanizma. |
| [Moderator pregleda komentar](#trigger-moderator-reviewed) | Moderator označi komentar kao pregledan. |
| [Moderator odobri komentar](#trigger-moderator-approved) | Moderator odobri komentar. |
| [Moderator označi kao spam](#trigger-moderator-spammed) | Moderator označi komentar kao spam. |
| [Moderator dodijeli značku](#trigger-moderator-awarded-badge) | Moderator dodijeli značku korisniku. |

### Više okidača po agentu

Agent se može pretplatiti na bilo koju kombinaciju okidača - [Predložak moderatora](#template-moderator) se, na primjer, pretplaćuje na oba `COMMENT_ADD` i `COMMENT_FLAG_THRESHOLD`. Svaki događaj aktivira agenta jednom s kontekstom tog događaja.

### Što sprječava aktiviranje agenta

Pretplaćeni događaj okidača **ne** aktivira agenta ako vrijedi bilo koje od sljedećeg:

- Agentov [status](#status-states) je **Onemogućen**.
- Agentov [URL ili opseg lokaliteta](#scope-url-locale) ne odgovara komentaru koji je izazvao događaj.
- Agentov [dnevni, mjesečni ili ograničeni budžet po stopi](#budgets-overview) je iscrpljen - okidač se bilježi kao **odbačen** s razlogom. Vidi [Razlozi odbacivanja](#drop-reasons).
- Istovremenost za ovog agenta je dosegnula maksimum (ograničeno po agentu).
- Tenant agenta ima nevaljanu naplatu.
- Akciju koja je izazvala događaj sam je napravio bot ili drugi agent (sprječavanje petlje).
- Okidač se odnosio na komentar koji je već ovaj agent obradio unutar prozora deduplikacije.

Kada se pretplaćeni okidač uspješno aktivira, agentova [Povijest izvršavanja](#run-history) prikazuje redak sa statusom **Pokrenuto** koji prelazi u **Uspjeh** ili **Pogreška** kada se izvršavanje završi.

### Pragovi glasova i prijava

Dva okidača - **Komentar prelazi prag glasova** i **Komentar prelazi prag prijava** - zahtijevaju numerički prag na obrascu za uređivanje. Okidač se aktivira u trenutku kad broj prijeđe konfiguriranu vrijednost (konkretnije, okidač praga prijava se aktivira kada `flagCount === flagThreshold`, pa izbor 1 znači "aktiviraj pri prvoj prijavi", a izbor 5 znači "aktiviraj kada stigne peta prijava").

### Odgođeni okidači

Bilo koji okidač može biti odgođen tako da agent pokrene kasnije, na primjer nakon što glasovi/prijave/odgovori imaju vremena da se stabiliziraju. Pogledajte [Odgođeni okidači](#trigger-deferred-delay).

### Sprječavanje petlji

Kako bi se spriječile beskonačne petlje, komentari **koje je napisao agent** nose `botId`. Okidači za nove komentare ignoriraju komentare s `botId`.

Konačni učinak: agenti mogu djelovati kao odgovor na *ljudske* akcije u vašem tenantu, ali akcije koje potječu od agenta nikada ne aktiviraju bilo koji agentov okidač. Ovo se odnosi na sve vrste okidača.

### REPLAY: interni okidač

Postoji i interni tip okidača `REPLAY` koji koristi značajka [Test Runs (Replays)](#test-runs-replays). Ne možete ga odabrati na obrascu za uređivanje - postoji kako bi se replay pokretanja označavala posebno u povijesti izvršavanja i isključivala iz prikaza živih pokretanja.