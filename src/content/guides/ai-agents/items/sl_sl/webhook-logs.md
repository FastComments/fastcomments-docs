Vsak agentni webhook ima svoj dnevnik dostave. Dosegljiv je z [webhook list page](https://fastcomments.com/auth/my-account/ai-agents/webhooks) prek gumba **Logs** v vsakem vrstici webhooka.

### Kaj je na strani

Stran vsebuje paginirano tabelo z eno vrstico na poskus dostave:

| Column | Meaning |
|---|---|
| When | Kdaj se je poskus zgodil. |
| Event | Tip dogodka (`trigger.succeeded`, `approval.requested`, itd.). |
| Status | Status dostave. |
| StatusCode | HTTP statusna koda, ki jo je vrnil vaš endpoint, kadar je na voljo. |
| Description | Kratek opis izida. Pri neuspelih dostavah, kjer ni bil prejet HTTP odgovor, je osnovno sporočilo o napaki shranjeno kot `{error: <message>}`. |

Stran podpira samo paginacijo - ni filtrov po statusu, tipu dogodka ali časovnem obdobju.

### Kaj lahko storite iz dnevnikov

- **Odločite, ali naj statusna koda spada med Statusne kode brez ponovitev.** Če vidite, da vaš endpoint nenehno vrača isto `4xx`, je to močan znak, da jo želite dodati med **No-retry status codes**, da platforma preneha poskušati znova.

### Podrobnosti o napakah

Ko dostava odpove brez HTTP odgovora (napaka DNS, povezava zavrnjena, časovna omejitev, TLS napaka itd.), je surovo sporočilo o napaki zabeleženo kot `{error: <message>}`. Platforma teh ne razvršča v imenovane kategorije, kot so `TIMEOUT` ali `DNS_ERROR` - preberite sporočilo o napaki neposredno, da vidite, kaj se je zgodilo.

Za HTTP odgovore stolpec StatusCode prikazuje kodo, ki jo je vrnil vaš endpoint. Pogosti primeri:

- **Vsi poskusi: `401` ali `403`** - vaš endpoint zavrača podpis. Preverite, da računate HMAC po `${timestamp}.${body}` in uporabljate pravilen secret. Oglejte si [Webhook Signing](#webhook-signing).
- **Vsi poskusi: `422`** - vaš endpoint meni, da je vsebina neveljavna. Bodisi popravite endpoint, ali pa dodajte `422` med **No-retry status codes**, če je zavrnitev pričakovana za nekatere dogodke.

### Kontekst posamezne dostave

Vsaka vnos v dnevniku vsebuje:

- `webhookId` - katera konfiguracija webhooka je ustvarila to dostavo.
- `agentId` - za katerega agenta je dostava.
- `triggerId` ali `approvalId` - osnovni zapis.
- `domain` - ujemajoča se domena.

Te podatke lahko uporabite za povezavo neuspele dostave z izvajanjem, na katerega se nanaša v [Run History](#run-history).

### Rok hrambe

Vnosi `AgentSyncLog` imajo enotno 1-leto TTL glede na `createdAt`, ne glede na izid - uspešne in neuspešne dostave se hranijo enako dolgo. Po poteku hrambe vnos v dnevniku izgine.

Če potrebujete dolgoročno revizijo, je vzdržen pristop ta: naj **sam endpoint** shrani dostave, ki jih prejme. To loči vaš revizijski dnevnik od politike hrambe platforme.

### Preizkusno pošiljanje

Gumb **Test send** v obrazcu za konfiguracijo webhooka zapiše lažno dostavo v isto tabelo dnevnika, tako da lahko preverite povezljivost od roba do roba, preden se zanesete na dejanske dogodke. Preizkusne dostave so v dnevniku jasno označene, da ne onesnažujejo statistike neuspehov v produkciji.

### Glej tudi

- [Webhooks Overview](#webhooks-overview).
- [Webhook Retries](#webhook-retries) za semantiko ponovnih poskusov, ki poganja te dnevnike.
- [Webhook Signing](#webhook-signing) za navodila, kako preveriti na vašem endpointu.