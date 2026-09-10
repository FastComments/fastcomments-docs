Webhooks können auch über die REST-API verwaltet werden. So abonnieren Integrationen wie Zapier Kommentarereignisse, ohne das Dashboard zu berühren, und es folgt dem REST Hooks‑Muster: abonnieren, Ereignisse empfangen, abbestellen.

API‑Abonnements existieren neben den im Dashboard konfigurierten Webhooks. Ein Kommentarereignis wird an den Dashboard‑Webhook für seine Domain und an jedes passende API‑Abonnement gesendet, jeweils als eigene Zustellung. Es gibt keine Beschränkung auf einen Abonnenten pro Ereignis.

## Authentifizierung

Jede Anfrage benötigt Ihren API‑Schlüssel im Header `x-api-key` (oder als Abfrageparameter `API_KEY`) und Ihre Mandanten‑ID im Abfrageparameter `tenantId`. Beide werden auf der Seite API‑Geheimnis im Dashboard angezeigt.

## Abonnieren

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Feld | Erforderlich | Beschreibung |
|-------|--------------|--------------|
| `url` | Ja | Eine absolute http- oder https-URL. |
| `event` | Ja | `comment-created`, `comment-updated` oder `comment-deleted`. |
| `domain` | Nein | Eine Domain aus Ihrer Kontokonfiguration. Standard ist `*`, was Ereignisse für jede Domain empfängt. |
| `method` | Nein | `POST` (Standard), `PUT` oder `DELETE`. |

Die Antwort enthält das Abonnement:

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

Das Abonnieren derselben URL für dasselbe Ereignis und dieselbe Domain gibt das bestehende Abonnement zurück, anstatt ein Duplikat zu erstellen, sodass ein Client sicher erneut versuchen kann. Jeder Mandant kann bis zu 50 API‑Abonnements haben.

## Auflisten

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Gibt jeden Webhook für den Mandanten zurück, einschließlich der im Dashboard verwalteten (`"source": "dashboard"`). Filtern Sie mit `event`, `domain` oder `source`.

## Abbestellen

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Das Löschen eines Abonnements verwirft auch alle noch für es in der Warteschlange befindlichen Ereignisse. Nur über die API erstellte Abonnements können auf diese Weise gelöscht werden. Dashboard‑Webhooks werden auf der Seite Webhooks bearbeitet.

## Nutzdaten und Signatur

Zustellungen verwenden dieselben Nutzdaten wie Dashboard‑Webhooks (siehe Datenstrukturen) und sind mit demselben HMAC‑Verfahren signiert (siehe Sicherheit & API‑Tokens). API‑Abonnements erhalten niemals den veralteten `token`‑Header, daher prüfen Sie stattdessen den Header `X-FastComments-Signature`.

## Antwort mit 410 Gone

Wenn der Endpunkt eines API‑Abonnements mit HTTP `410 Gone` antwortet, behandelt FastComments dies als Abbestellung: Das Abonnement wird zusammen mit seinen wartenden Ereignissen gelöscht und es werden keine weiteren Zustellungen versucht. In dem Dashboard konfigurierten Webhooks werden niemals automatisch gelöscht; für sie bedeutet ein 410 einen normalen Fehler. Jeder andere Fehlstatus wird erneut versucht und deaktiviert schließlich den Webhook, wie in Funktionsweise & Umgang mit Wiederholungen beschrieben.

## Dashboard

API‑Abonnements werden auf der Seite Webhooks unter der Domain, für die sie erstellt wurden, aufgelistet, wo ein Administrator sie deaktivieren, wieder aktivieren oder löschen kann.

---