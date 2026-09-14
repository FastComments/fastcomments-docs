[Val Town](https://val.town) pokreće TypeScript na Deno, pa je val pravi server. To ga čini dobrim izborom za FastComments: widget je script tag na stranici, a sve što zahteva tajnu, poput Secure SSO ili verifikacije webhook‑a, može se izvršavati na serveru u istom val‑u.

Ovaj vodič pokriva dodavanje widgeta za komentare u HTTP val, prikazivanje broja komentara na indeksnoj stranici, prijavljivanje korisnika pomoću Val Town naloga koji već imaju, i primanje webhook‑ova za komentare.

Ne morate imati nalog da biste ga isprobali. Primeri koriste `tenantId: "demo"`, zajednički sandbox, a korak 2 objašnjava kako preći na svoj.