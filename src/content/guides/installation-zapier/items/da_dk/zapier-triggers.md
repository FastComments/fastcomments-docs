## Udløsere

Udløsere starter en Zap, når noget sker i FastComments. Alle tre er øjeblikkelige: FastComments leverer begivenheden til Zapier via et webhook, så snart den sker. Intet forespørger din konto, og ingen API‑kreditter bruges på at vente.

| Udløser | Udløses når |
|---------|-------------|
| Ny kommentar | En kommentar er postet. Som standard udløses kun godkendte, ikke‑spam kommentarer. |
| Opdateret kommentar | En kommentar er redigeret, godkendt, stemt på, fastgjort, låst eller på anden måde ændret. |
| Slettet kommentar | En kommentar er slettet. |

Hver udløser returnerer den fulde kommentar: id, side‑URL og URL‑ID, kommentators navn og e‑mail, kommentarteksten som markdown og som HTML, stemmetal, godkendelses‑ og spam‑flag, lokalet, domænet og eventuelle nævnelser. Felterne svarer til webhook‑payloaden dokumenteret under Webhooks, Data Structures.

## Indstillinger

**Domæne.** Hver udløser har et valgfrit domænefilter, der viser de domæner, der er konfigureret på din konto. Lad den stå tom for at modtage begivenheder fra alle domæner.

**Inkluder ikke‑godkendte og spam‑kommentarer.** Kun på udløseren Ny kommentar. Kommentarer, der holdes til moderation eller er markeret som spam, springes over som standard. Når en sådan kommentar godkendes senere, udløses opdateret‑kommentar‑udløseren for den, så en Zap, der skal reagere på hver kommentar, der bliver synlig, bruger Opdateret kommentar med et filter på godkendelses‑feltet.

## Sådan fungerer levering

At aktivere en Zap opretter et webhook‑abonnement på din konto, som er synligt på Webhooks‑siden med kilden **API**. At deaktivere Zap’en fjerner det. Zapier's egne grænser gælder for, hvor mange begivenheder den accepterer pr. minut; FastComments forsøger at levere igen, hvis leveringen fejler, med en stigende forsinkelse, og deaktiverer et abonnement, der fortsat fejler i seks dage. Et deaktiveret abonnement kan genaktiveres fra Webhooks‑siden, eller du kan blot slå Zap’en fra og til igen for at oprette et nyt.

En konto kan have op til 50 API‑abonnementer. Hver Zap, der bruger en FastComments‑udløser, bruger ét.