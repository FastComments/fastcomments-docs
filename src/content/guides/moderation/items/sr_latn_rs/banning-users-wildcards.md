Moguće je blokirati korisnike koji koriste određene provajdere e‑mailova koristeći džokere.

Na primer, ako otkrijete da su svi komentari sa **@bademail.com** spam, možete jednostavno blokirati celog provajdera e‑maila unosom "*@bademail.com" u polje za e‑mail prilikom dodavanja blokiranog korisnika.

Obratite pažnju na „*“ pre @ u e‑mail adresi.

### Subdomains

Zabrana domena takođe pokriva svaki poddomen tog domena. Blokiranje `*@bademail.com` takođe blokira `someone@mail.bademail.com` i `someone@eu.mail.bademail.com`, pa nije potrebno dodavati zasebnu zabranu za svaki poddomen.

Ako želite da blokirate samo određeni poddomen, unesite taj poddomen, na primer `*@mail.bademail.com`. Ta zabrana ne utiče na `someone@bademail.com`.

### Banning a Domain From a Comment

Ne morate ručno unositi obrazac. Kada blokirate korisnika iz komentara na stranici Moderate Comments page, dijalog za blokiranje ima polje za potvrdu "Ban All @domain Users" koje kreira istu `*@domain` zabranu za domen e‑mail adrese komentatora.

### Supported Patterns

Jedini podržani oblik džokera je pojedinačni `*` umesto celog dela imena, praćen `@` i domenom. Ostali oblici se odbacuju kada pokušate da ih sačuvate:

- `*@*.bademail.com` nije potreban, jer `*@bademail.com` već pokriva poddomene.
- `name*@bademail.com` i `*bademail.com` nisu podržani.