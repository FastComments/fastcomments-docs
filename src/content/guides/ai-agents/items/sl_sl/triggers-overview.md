A **sprožilec** je dogodek, ki aktivira agenta. Vsak agent ima lahko definiran enega ali več sprožilcev.

### Celoten seznam

| Sprožilec | Kdaj se sproži |
|---|---|
| [Comment Added](#trigger-comment-add) | Objavljen je nov komentar. |
| [Comment Edited](#trigger-comment-edit) | Komentar je urejen. Prejšnje besedilo je vključeno v kontekst agenta. |
| [Comment Deleted](#trigger-comment-delete) | Komentar je izbrisan. |
| [Comment Pinned](#trigger-comment-pin) | Komentar je pripet (s strani kogarkoli, vključno z moderatorjem ali drugim agentom). |
| [Comment Unpinned](#trigger-comment-unpin) | Komentar je odpet. |
| [Comment Locked](#trigger-comment-lock) | Komentar je zaklenjen (ni dovoljenih nadaljnjih odgovorov). |
| [Comment Unlocked](#trigger-comment-unlock) | Komentar je odklenjen. |
| [Comment Crosses Vote Threshold](#trigger-comment-vote-threshold) | Neto glasovi komentarja dosežejo konfigurirano mejo. |
| [Comment Crosses Flag Threshold](#trigger-comment-flag-threshold) | Število prijav komentarja doseže natanko konfigurirano mejo. |
| [User Posts First Comment](#trigger-new-user-first-comment) | Uporabnik objavi svoj prvi komentar na tem mestu. |
| [Comment Auto-Spammed](#trigger-comment-auto-spammed) | Komentar je avtomatsko označen kot neželena vsebina s strani spam mehanizma. |
| [Moderator Reviews Comment](#trigger-moderator-reviewed) | Moderator označi komentar kot pregledan. |
| [Moderator Approves Comment](#trigger-moderator-approved) | Moderator odobri komentar. |
| [Moderator Marks Spam](#trigger-moderator-spammed) | Moderator označi komentar kot spam. |
| [Moderator Awards Badge](#trigger-moderator-awarded-badge) | Moderator podeli značko uporabniku. |

### Več sprožilcev na agenta

Agent se lahko naroči na katerokoli kombinacijo sprožilcev - [Moderator template](#template-moderator) se na primer naroči tako na `COMMENT_ADD` kot na `COMMENT_FLAG_THRESHOLD`. Vsak dogodek sproži agenta enkrat s kontekstom tega dogodka.

### Kaj preprečuje sprožitev agenta

Naročeni sprožilec ne sproži agenta, če velja katera od naslednjih situacij:

- Agentov [status](#status-states) je **Onemogočen**.
- Agentov [obseg URL ali lokalne nastavitve](#scope-url-locale) se ne ujema s sprožilnim komentarjem.
- Agentov [dnevni, mesečni ali proračun za omejitev hitrosti](#budgets-overview) je porabljen - sprožilec je zabeležen kot **opuščen** z razlogom. Glejte [Drop Reasons](#drop-reasons).
- Sočasnost za tega agenta je presežena (omejeno na agenta).
- Najemnik agenta ima neveljaven obračun.
- Sprožilno dejanje je bilo samo izvedeno s strani bota ali drugega agenta (preprečevanje zanke).
- Sprožilec se je nanašal na komentar, ki ga je ta agent že obdelal znotraj okna deduplikacije.

Ko se naročeni sprožilec uspešno sproži, agentov [Run History](#run-history) prikaže vrstico s statusom **Začeto**, ki se ob zaključku zagona spremeni v **Uspeh** ali **Napaka**.

### Meje glasov in prijav

Dva sprožilca - **Comment Crosses Vote Threshold** in **Comment Crosses Flag Threshold** - zahtevata numerično mejo na obrazcu za urejanje. Sprožilec se sproži v trenutku, ko števec preseže konfigurirano vrednost (natančneje, sprožilec za prag prijav se sproži, ko `flagCount === flagThreshold`, zato izbira 1 pomeni "sproži ob prvi prijavi", izbira 5 pa pomeni "sproži, ko prispe peta prijava").

### Zamaknjeni sprožilci

Poljuben sprožilec je mogoče zamakniti, tako da agent zažene kasneje, na primer potem, ko imajo glasovi/prijave/odgovori čas, da se umirijo. Glejte [Deferred Triggers](#trigger-deferred-delay).

### Preprečevanje zank

Da bi preprečili neskončne zanke, komentarji, ki jih **zapiše agent**, nosijo `botId`. Sprožilci za nove komentarje prezrejo komentarje z `botId`.

Splošni učinek: agenti lahko ukrepajo kot odgovor na *človeška* dejanja v vašem najemniku, vendar dejanja, ki jih ustvarijo agenti, nikoli ne sprožijo agentnih sprožilcev. To velja za vse vrste sprožilcev.

### REPLAY: notranji sprožilec

Obstaja tudi notranja vrsta sprožilca `REPLAY`, ki jo uporablja funkcija [Test Runs (Replays)](#test-runs-replays). Na obrazcu za urejanje je ne morete izbrati - obstaja zato, da so ponovitve označene ločeno v zgodovini zaganjanj in izključene iz prikazov v živo.