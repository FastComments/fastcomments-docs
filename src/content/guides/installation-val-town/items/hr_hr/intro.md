[Val Town](https://val.town) pokreće TypeScript na Deno-u, pa je val pravi poslužitelj. To ga čini dobrim izborom za FastComments: widget je script tag na stranici, a sve što treba tajnu, poput Secure SSO ili provjere webhooka, može se izvoditi na poslužiteljskoj strani u istom valu.

Ovaj vodič pokriva dodavanje widgeta za komentare u HTTP val, prikaz broja komentara na indeksnoj stranici, prijavu korisnika pomoću Val Town računa koji već imaju i primanje webhookova za komentare.

Ne trebate račun za probu. Primjeri koriste `tenantId: "demo"`, zajednički sandbox, a korak 2 opisuje prebacivanje na vlastiti.