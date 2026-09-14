Jednom kada isključite `demo` tenant, widget može odbiti učitavanje sa greškom autorizacije. To je zato što FastComments ne zna da treba da dozvoli da se vaš nalog koristi na tom domenu.

[Idite ovde da dodate svoj sajt na vaš nalog.](https://fastcomments.com/auth/my-account/configure-domains)

Val Town vredi još jednom pogledati, jer se val može pristupiti sa više od jednog host imena:

- Svaki HTTP val ima dugačak podrazumevani endpoint, `<org>--<id>.web.val.run`.
- Rezervisanje prilagođenog poddomena dodaje `<name>.val.run`.
- A [custom domain](https://docs.val.town/vals/http/custom-domains/) adds a third.
- Grane dobijaju svoje URL‑ove.

Dodajte ona host imena sa kojih zaista servirate widget. Ako potražite poddomen nakon što ste postavili stvari, dodajte i to, inače widget radi na starom URL‑u i ne uspeva na novom.