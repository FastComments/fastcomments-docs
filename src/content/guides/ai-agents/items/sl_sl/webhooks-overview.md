Agent webhooki so HTTP povratni klici, ki jih platforma sproži, ko se agentova izvedba zaključi ali ko se stanje odobritve spremeni. Uporabite jih lahko za posredovanje aktivnosti agentov v lastne sisteme - nadzorne plošče za moderiranje, revizijske dnevnike, Slack kanale, orodja za eskalacijo.

Konfigurirajo se pod zavihkom **Webhooks** na [strani AI Agents](https://fastcomments.com/auth/my-account/ai-agents).

### Kaj se dostavi

Štiri vrste dogodkov:

- **`trigger.succeeded`** - izvedba agenta se je uspešno zaključila.
- **`trigger.failed`** - izvedba agenta je javila napako.
- **`approval.requested`** - dejanje je bilo postavljeno v vrsto za človeško odobritev.
- **`approval.decided`** - odobritev je bila odobrena, zavrnjena ali je izvedba spodletela.

Glejte [Webhook Events](#webhook-events) za to, kdaj se kateri dogodki sprožijo, in [Webhook Payloads](#webhook-payloads) za shemo vsakega od njih.

### Kaj se ne dostavi

- **Webhooki za posamezna dejanja orodij.** Izvedba, ki pokliče `pin_comment`, ne sproži ločenega webhooka za pripinjanje - dejanje je vključeno v payload `trigger.succeeded` izvedbe. Če želite dostavo na ravni dejanj, razčlenite polje `actions` v payloadu sprožilca.
- **Zavrgljeni sprožilci.** Sprožilec, ki se ne izvrši (presežen proračun, napačen obseg), ne sproži webhooka. Zavrgelne so vidne samo na [strani Analytics](#analytics-page).
- **Sprožilci, ustvarjeni s predvajanjem.** Testne izvedbe ne sprožijo webhookov. Glejte [Test Runs (Replays)](#test-runs-replays).

### Konfiguracija

Za vsak webhook nastavite:

- **URL** - končna točka HTTPS, na katero se pošiljajo POST zahteve.
- **Domain** - domena komentarjev, na katero se ta webhook nanaša (vaš najemnik lahko gosti več domen). `*` ustreza vsem domenam; `*.example.com` je glob; natančna domena ustreza samo njej sami.
- **Events** - na katere od štirih vrst dogodkov se naročite.
- **Agents** - prazno pomeni "vsi agenti", ali pa seznam določenih ID-jev agentov za filtriranje.
- **Method** - POST ali PUT (privzeto POST).
- **No-retry status codes** - kode odgovorov HTTP, ki naj se obravnavajo kot končne napake in ne poskušajo znova (npr. 410 Gone, 422 Unprocessable). Glejte [Webhook Retries](#webhook-retries).

Več webhookov se lahko naroči na isti dogodek - vsak se čaka v svoji vrsti in se dostavi na svoj URL.

### Ujemanje po domenah

Določen dogodek se dostavi **vsakemu webhooku, katerega polje `domain` ustreza domeni komentarja**. Ujemanje uporablja preprost glob:

- Natančno: `example.com` ustreza samo `example.com`.
- Zvezdica: `*` ustreza vsem domenam.
- Glob za poddomene: `*.example.com` ustreza `blog.example.com`, `forum.example.com`, ne pa tudi `example.com` sami.

Domena v payloadu je prvi rezultat, ki ni ničelen, od: domene komentarja, URL-ja, razčlenjenega glede na konfiguracijo domen vašega najemnika, ali iskanja `Page` po `urlId`.

### Filtriranje po agentih

Polje **Agents** webhooku omogoča naročnino samo na določene agente. Prazno pomeni "vsi agenti". Ko ni prazno, se webhook sproži samo za agente na seznamu.

To je uporabno, kadar imate en webhook za dogodke moderiranja in drugega za dogodke angažiranosti, ki usmerjata v različne sisteme.

### Testna pošiljka

Vmesnik za konfiguracijo webhookov ima gumb **Test send**, ki na URL pošlje lažni payload, da lahko preverite povezljivost, podpisovanje in kodo odgovora vaše končne točke, ne da bi čakali na pravi dogodek.

### Dnevniki dostave

Vsaka dostava (in vsak ponovni poskus) se zapiše na stran [Webhook Delivery Logs](#webhook-logs), da lahko vidite, kaj se je zgodilo na povezavi.

### Podpisovanje

Vsak webhook je podpisan z HMAC-SHA256 z uporabo API skrivnosti vašega najemnika. Glejte [Webhook Signing](#webhook-signing).

### Upravičenost

Agent webhooki zahtevajo veljavno obračunavanje na najemniku. Uporabljajo isto infrastrukturo podpisovanja/skrivnosti kot vaši obstoječi webhooki za komentarje - če ste že integrirali webhooka za komentarje (glejte [vodnik Webhooks](/guide-webhooks.html)), ima integracija agent webhookov enako obliko z novimi vrstami dogodkov.