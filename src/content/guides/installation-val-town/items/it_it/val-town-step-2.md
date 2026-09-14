`tenantId: "demo"` è un sandbox pubblico condiviso. Funziona senza registrazione, motivo per cui gli esempi lo usano, ma tutti gli altri che provano FastComments scrivono negli stessi thread e chiunque può moderarli. Cambia prima di pubblicare qualsiasi cosa a cui tieni.

Il tuo tenant ID si trova nella [pagina del segreto API](https://fastcomments.com/auth/my-account/api-secret).

Un tenant ID è pubblico e appartiene al codice del browser. Un segreto API no, e nulla in questa pagina ne richiede uno.

## Leggilo da una variabile d'ambiente

I valori di Val Town sono pubblici nel livello gratuito, quindi la loro origine è leggibile da tutti. Conserva tutto ciò che è sensibile nelle variabili d'ambiente, leggile con `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Only accounts created on eu.fastcomments.com set this, to "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

Questo è più importante del solito su Val Town per un secondo motivo: **remixare un val copia le chiavi delle variabili d'ambiente, ma non i loro valori.** Un segreto conservato in una variabile d'ambiente non segue il tuo val nell'account di qualcun altro. Un segreto scritto in un file lo fa.

Tornare a `"demo"` mantiene il val funzionante per chiunque lo remix prima di impostare il proprio tenant.

## Account EU

Un account, i suoi dati e le sue chiavi vivono in una sola regione. Se il tuo è stato creato su `eu.fastcomments.com`, ogni configurazione del widget necessita anche di `region: "eu"`, e gli script si caricano da `cdn-eu.fastcomments.com`. Altrimenti lascia entrambi così.

---