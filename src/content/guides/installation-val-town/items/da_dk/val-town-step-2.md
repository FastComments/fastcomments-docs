`tenantId: "demo"` er en delt offentlig sandbox. Den fungerer uden tilmelding, hvilket er grunden til at eksemplerne bruger den, men alle andre der prøver FastComments skriver i de samme tråde, og enhver kan moderere dem. Skift, før du publicerer noget, du har brug for.

Din tenant‑ID er på [API‑hemmelighedssiden](https://fastcomments.com/auth/my-account/api-secret).

Et tenant‑ID er offentligt og hører hjemme i browserkoden. En API‑hemmelighed gør det ikke, og intet på denne side kræver en.

## Læs den fra en miljøvariabel

Val Town‑værdier er offentlige på den gratis plan, så deres kilde er læsbar for alle. Gem alt følsomt i miljøvariabler, læs med `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Only accounts created on eu.fastcomments.com set this, to "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

Dette er vigtigere end normalt på Val Town af en anden grund: **remixing af en val kopierer miljøvariabel‑nøgler, men ikke deres værdier.** En hemmelighed gemt i en miljøvariabel følger ikke din val ind i en andens konto. En hemmelighed skrevet i en fil gør.

At falde tilbage til `"demo"` holder val’en i funktion for enhver, der remixer den, før de indstiller deres egen tenant.

## EU‑konti

En konto, dens data og dens nøgler lever i én region. Hvis din blev oprettet på `eu.fastcomments.com`, skal hver widget‑konfiguration også have `region: "eu"`, og scriptet indlæses fra `cdn-eu.fastcomments.com`. Ellers lad begge være uændrede.

---