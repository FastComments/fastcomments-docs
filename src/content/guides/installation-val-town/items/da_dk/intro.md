[Val Town](https://val.town) kører TypeScript på Deno, så en val er en rigtig server. Det gør den velegnet til FastComments: widget'en er et script‑tag på siden, og alt der har brug for en hemmelighed, som Secure SSO eller verifikation af en webhook, kan køre server‑side i den samme val.

Denne guide dækker, hvordan du tilføjer kommentarswidget'en til en HTTP‑val, viser kommentarantal på en indeks‑side, logger brugere ind med den Val Town‑konto, de allerede har, og modtager kommentar‑webhooks.

Du behøver ikke en konto for at prøve det. Eksemplerne bruger `tenantId: "demo"`, en delt sandbox, og trin 2 dækker, hvordan du skifter til din egen.