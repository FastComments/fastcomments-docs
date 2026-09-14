`tenantId: "demo"` ist ein gemeinsam genutzter öffentlicher Sandbox. Sie funktioniert ohne Anmeldung, weshalb die Beispiele sie verwenden, aber alle anderen, die FastComments ausprobieren, schreiben in dieselben Threads und jeder kann sie moderieren. Wechseln Sie, bevor Sie etwas veröffentlichen, das Ihnen wichtig ist.

Ihre Tenant-ID finden Sie auf der [API-Geheimnis-Seite](https://fastcomments.com/auth/my-account/api-secret).

Eine Tenant-ID ist öffentlich und gehört in den Browser‑Code. Ein API‑Geheimnis ist das nicht, und nichts auf dieser Seite benötigt eines.

## Aus einer Umgebungsvariable lesen

Val Town Vals sind im kostenlosen Tarif öffentlich, sodass ihr Quellcode weltlesbar ist. Bewahren Sie alles Sensible in Umgebungsvariablen auf, lesen Sie sie mit `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Only accounts created on eu.fastcomments.com set this, to "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

Das ist auf Val Town aus einem zweiten Grund wichtiger als üblich: **Beim Remix eines Vals werden Umgebungsvariablen‑Schlüssel kopiert, aber nicht deren Werte.** Ein in einer Umgebungsvariable aufbewahrtes Geheimnis folgt Ihrem Val nicht in das Konto eines anderen. Ein in einer Datei geschriebenes Geheimnis tut es.

Das Zurückfallen auf `"demo"` lässt den Val für jeden funktionieren, der ihn remixt, bevor er seinen eigenen Tenant festlegt.

## EU-Konten

Ein Konto, seine Daten und seine Schlüssel befinden sich in einer Region. Wenn Ihres auf `eu.fastcomments.com` erstellt wurde, benötigt jede Widget‑Konfiguration ebenfalls `region: "eu"`, und die Skripte werden von `cdn-eu.fastcomments.com` geladen. Andernfalls lassen Sie beide unverändert.

---