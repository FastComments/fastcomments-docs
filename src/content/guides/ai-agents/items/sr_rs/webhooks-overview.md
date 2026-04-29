Agent webhooks су HTTP повратни позиви које платформа шаље када се извршавање агента заврши или када се статус одобрења промени. Користите их да преусмерите активност агента у своје системе — контролне табле за модерацију, логове ревизије, Slack канале, алате за ескалацију.

Конфигуришу се под картицом **Webhooks** на [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents).

### What gets delivered

Четири типа догађаја:

- **`trigger.succeeded`** - извршавање агента је успешно завршено.
- **`trigger.failed`** - ток извршавања агента је пријавио грешку.
- **`approval.requested`** - акција је стављена у ред за људско одобрење.
- **`approval.decided`** - одобрење је одобрено, одбијено или извршавање је пропало.

Види [Webhook Events](#webhook-events) за то који догађаји се када шаљу, и [Webhook Payloads](#webhook-payloads) за шему сваког од њих.

### What's not delivered

- **Per-tool-action webhooks.** Покретање које позива `pin_comment` не покреће одвојени webhook за сам pin — акција је укључена у payload покрета `trigger.succeeded`. Ако желите испоруку по акцији, парсирајте `actions` низ у trigger payload-у.
- **Dropped triggers.** Тригер који се не диспатчује (прекорачење буџета, погрешан опсег) не шаље webhook. Drop-ови су видљиви само на [Analytics page](#analytics-page).
- **Replay-produced triggers.** Тест покретања не шаљу webhook-ове. Види [Test Runs (Replays)](#test-runs-replays).

### Configuration

За сваки webhook који подесите:

- **URL** - HTTPS крајња тачка на коју се ради POST.
- **Domain** - домен коментара на који се овај webhook примењује (your tenant may host multiple domains). `*` поклапа све домене; `*.example.com` је глоб; тачан домен поклапа само тај.
- **Events** - који од четири типа догађаја да буду претплаћени.
- **Agents** - празно значи "сви агенти", или листа специфичних agent ID-јева за филтрирање.
- **Method** - POST или PUT (подразумевано POST).
- **No-retry status codes** - HTTP статус кодови које треба третирати као коначне грешке, без поновних покушаја (нпр. 410 Gone, 422 Unprocessable). Види [Webhook Retries](#webhook-retries).

Више webhook-ова може бити претплаћено на исти догађај — сваки се ставља у ред независно и испоручује се на свој URL.

### Per-domain matching

Дати догађај се испоручује на **свaki webhook чије `domain` поље поклапа домен коментара**. Поклапање користи једноставан глоб:

- Тачно: `example.com` поклапа само `example.com`.
- Јокер звездица: `*` поклапа сваки домен.
- Поддоменски глоб: `*.example.com` поклапа `blog.example.com`, `forum.example.com`, али не и сам `example.com`.

Домен у payload-у је први не-null резултат из: коментарског `domain`, URL-а парсираног према your tenant's domain configuration, или `Page` lookup-а по `urlId`.

### Per-agent filtering

Поље **Agents** омогућава webhook-у да се претплати само на одређене агенте. Празно значи "сви агенти". Када није празно, webhook се активира само за агенте из листе.

Ово је корисно када имате један webhook за догађаје модерације и други за догађаје ангажовања, оба усмерена ка различитим downstream системима.

### Test send

UI за конфигурацију webhook-а има дугме **Test send** које шаље лажни payload на URL тако да можете верификовати конективност, потписивање и одговорни код вашег endpoint-а без чекања правог догађаја.

### Delivery logs

Свака испорука (и сваки поновни покушај) завршава у [Webhook Delivery Logs](#webhook-logs) страници тако да можете видети шта се догодило на мрежи.

### Signing

Сваки webhook се потписује HMAC-SHA256 користећи your tenant's API secret. Види [Webhook Signing](#webhook-signing).

### Eligibility

Agent webhooks захтевају важеће плаћање на tenant-у. Користе исту инфраструктуру за потписивање/секрете као и ваши постојећи comment webhooks — ако сте већ интегрисали comment webhooks (види [Webhooks guide](/guide-webhooks.html)), интеграција agent webhook-ова има исти облик са новим типовима догађаја.