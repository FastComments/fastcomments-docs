Agent webhooki so HTTP povratni klici, ki jih platforma sproži, ko se izvedba agenta zaključi ali ko se stanje odobritve spremeni. Uporabite jih za posredovanje dejavnosti agenta v svoje sisteme - nadzorne plošče za moderiranje, revizijske dnevnike, kanale Slack, orodja za eskalacijo.

Nastavijo se pod zavihkom **Webhooks** na strani [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents).

### What gets delivered

Štiri vrste dogodkov:

- **`trigger.succeeded`** - izvedba agenta se je uspešno zaključila.
- **`trigger.failed`** - izvedba agenta je povzročila napako.
- **`approval.requested`** - dejanje je bilo uvrščeno v čakalno vrsto za ročno odobritev.
- **`approval.decided`** - odobritev je bila odobrjena, zavrnjena ali pa je izvedba spodletela.

Oglejte si [Webhook Events](#webhook-events), kdaj kateri dogodki sprožijo, in [Webhook Payloads](#webhook-payloads) za shemo vsakega.

### What's not delivered

- **Per-tool-action webhooks.** Zagon, ki pokliče `pin_comment`, ne sproži ločenega webhooka za pripenjanje - dejanje je vključeno v `trigger.succeeded` payload izvedbe. Če želite dostavo za posamezno dejanje, razčlenite polje `actions` v triggerjevem payloadu.
- **Dropped triggers.** Sprožilec, ki se ne izvede (prekoračen proračun, napačen obseg), ne sproži webhooka. Izpadi so vidni samo na strani [Analytics page](#analytics-page).
- **Replay-produced triggers.** Testni teki ne sprožijo webhookov. Oglejte si [Test Runs (Replays)](#test-runs-replays).

### Configuration

Za vsak webhook, ki ga nastavite:

- **URL** - HTTPS končna točka, na katero se pošlje POST.
- **Domain** - domena komentarjev, na katero se ta webhook nanaša (vaš najemnik lahko gosti več domen). `*` ustreza vsem domenam; `*.example.com` je glob; natančna domena ustreza samo tej eni.
- **Events** - na katere od štirih vrst dogodkov se naročiti.
- **Agents** - prazno pomeni "all agents", ali seznam določenih ID-jev agentov za filtriranje.
- **Method** - POST ali PUT (privzeto POST).
- **No-retry status codes** - HTTP odzivni kod, ki jih je treba obravnavati kot končne napake in jih ne ponavljati (npr. 410 Gone, 422 Unprocessable). Oglejte si [Webhook Retries](#webhook-retries).

Več webhookov se lahko naroči na isti dogodek - vsak se postavi v čakalno vrsto neodvisno in se dostavi na svoj URL.

### Per-domain matching

Dani dogodek se dostavi **vsem webhookom, katerih polje `domain` se ujema z domeno komentarja**. Ujemanje uporablja preprost glob:

- Exact: `example.com` se ujema samo z `example.com`.
- Wildcard star: `*` ustreza vsaki domeni.
- Subdomain glob: `*.example.com` ustreza `blog.example.com`, `forum.example.com`, vendar ne `example.com` samemu.

Domena v payloadu je prvi nenull rezultat iz: `domain` komentarja, URL analiziran glede na konfiguracijo domen vašega najemnika, ali iskanje `Page` po `urlId`.

### Per-agent filtering

Polje **Agents** omogoča, da se webhook naroči samo na določene agente. Prazno pomeni "vsi agenti". Ko ni prazno, se webhook sproži samo za agente s seznama.

To je uporabno, ko imate en webhook za dogodke moderiranja in drugega za dogodke angažiranosti, oba pa usmerjata v različne sisteme v ozadju.

### Test send

Vmesnik za konfiguracijo webhookov ima gumb **Test send**, ki pošlje lažen payload na URL, da lahko preverite povezljivost, podpisovanje in odzivno kodo vaše končne točke, brez čakanja na resničen dogodek.

### Delivery logs

Vsaka dostava (in vsak ponovni poskus) se zabeleži na strani [Webhook Delivery Logs](#webhook-logs), tako da lahko vidite, kaj se je zgodilo na omrežju.

### Signing

Vsak webhook je podpisan z HMAC-SHA256 z uporabo API skrivnosti vašega najemnika. Oglejte si [Webhook Signing](#webhook-signing).

### Eligibility

Agent webhooki zahtevajo veljavno obračunavanje na najemniku. Uporabljajo isto infrastrukturo podpisovanja/skrivnosti kot vaši obstoječi webhooki za komentarje - če ste že integrirali comment webhooks (glejte [Webhooks guide](/guide-webhooks.html)), je integracija agent webhookov enake oblike z novimi vrstami dogodkov.