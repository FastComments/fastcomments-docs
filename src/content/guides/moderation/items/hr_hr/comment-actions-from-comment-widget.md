Dio akcija moderiranja može se izvršiti izravno u niti komentara, bez potrebe za odlaskom na stranicu Moderacija komentara.

Kad ste prijavljeni, kliknite gumb za uređivanje u gornjem desnom kutu komentara. Kao moderator trebali biste imati sljedeće opcije:

- **Pin** that comment
- **Delete** that comment
- **Delete** that comment + **Ban the user** (Permanent or Shadow, more details later)
- **Edit** that comment
- **Lock** or **Unlock** that comment (more details below)
- Mark that comment **Approved** (show it) or **Not Approved** (hide it)
- Mark that comment as **Spam** or **Not Spam**

### Zaključavanje komentara

Zaključavanje pojedinačnog komentara sprječava bilo kakve nove odgovore na njega, a također sprječava da se sam komentar uređuje ili briše dok nije otključan. Ovo se odnosi na sve, uključujući administratore i moderatore. Ako trebate urediti ili ukloniti zaključani komentar, prvo ga otključajte, napravite promjenu i po želji ga ponovno zaključajte.

Ikona lokota pojavljuje se u gornjem desnom kutu zaključanog komentara kako bi čitatelji na prvi pogled vidjeli da je nit zatvorena. Unosi izbornika Uredi i Izbriši su skriveni za zaključane komentare i u widgetu komentara i u javnom API-ju (`PATCH` i `DELETE` vraćaju `code: 'locked'` ako se pozovu nad zaključanim komentarom).

Dvije namjerne iznimke zaobilaze zaključavanje, jer bi inače ostavile siroče podatke: kada korisnik izbriše svoj cijeli račun (njihovi komentari se raščiste bez obzira na stanje zaključavanja), i kada moderator zabrani korisnika s opcijom "izbriši sve komentare od ovog korisnika" (sweeping čisti zaključavanja).

### Zatvaranje niti komentara

Moderatori i administratori mogu zaključati, odnosno zatvoriti, niti komentara odabirom `Close Thread` u izborniku s tri točke na vrhu područja komentara, ako su prijavljeni. Kasnije, u bilo kojem trenutku, mogu odabrati `Re-Open Thread` kako bi ponovno omogućili komentiranje.

Zatvaranje niti komentara onemogućuje nove komentare, ali i dalje dopušta glasanje i korisnicima da, ako žele, izbrišu svoje komentare.

Zatvaranje i ponovno otvaranje niti komentara odmah utječe na sve korisnike koji gledaju nit.

Možete također označiti nit kao samo za čitanje (read-only) što uklanja opcije glasanja i brisanja, stvaranjem pravila prilagodbe posebno za tu stranicu.

### Ažurirano uživo

Sve ove radnje će odmah ažurirati niti komentara drugih korisnika bez potrebe da oni osvježe stranicu. Međutim, moderacijske radnje poput skrivanja komentara ili označavanja kao spam, ne uklanjaju komentar s **moderatorovog** zaslona kako bi, ako je potrebno, brzo mogli poništiti radnju. Da bi se naznačilo da je komentar skriven, bit će istaknut u usporedbi s ostalim komentarima (boja isticanja ovisi o razlogu uklanjanja).

For example, given users `A (commenter)`, `B (Moderator 1)`, and `C (Moderator 2)`.

...and the following scenario:

1. `User B (Moderator 1)` hides a comment.
2. For `User A (commenter)` that comment is immediately hidden.
3. For `User C (Moderator 2)` that comment is immediately hidden.
4. For the user that made the change, `User B (Moderator 1)`, the comment remains on their screen, but is highlighted as removed. They have the option to undo their action, in which case the other users will see the update, live, again.