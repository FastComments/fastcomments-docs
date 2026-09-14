Kad isključite `demo` najam, widget može odbiti učitavanje s greškom autorizacije. To je zato što FastComments ne zna da bi trebao dopustiti da se vaš račun koristi na toj domeni.

[Idite ovdje da dodate svoju stranicu na svoj račun.](https://fastcomments.com/auth/my-account/configure-domains)

Val Town zaslužuje još jedan pogled ovdje, jer se val može pristupiti s više od jednog host imena:

- Svaki HTTP val ima dugačak zadani endpoint, `<org>--<id>.web.val.run`.
- Rezerviranje prilagođenog poddomena dodaje `<name>.val.run`.
- A [custom domain](https://docs.val.town/vals/http/custom-domains/) adds a third.
- Granice dobivaju svoje URL-ove.

Dodajte sve host imena s kojih stvarno poslužujete widget. Ako rezervirate poddomen nakon što ste postavili sve, dodajte i to, inače widget radi na starom URL-u i ne uspijeva na novom.