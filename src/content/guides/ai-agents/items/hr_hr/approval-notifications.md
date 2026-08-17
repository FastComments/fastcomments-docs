When the agent queues an approval, the platform notifies reviewers via email. Two settings on the edit form control this: **who** is notified and **how often**.

### Tko: način obavijesti

Two modes:

- **Svi administratori i moderatori** (zadano) - svaki vlasnik računa, super administrator i administrator moderiranja komentara na najmodavcu je kandidat recenzent.
- **Specifični korisnici** - ručno odaberite popis iz dvostrukog odabira na obrascu za uređivanje.

Either way, a candidate reviewer must have an account on the tenant and a valid email address to receive notifications.

### Koliko često: učestalost po korisniku

Each candidate reviewer's **own profile** sets their personal notification frequency for agent approvals:

- **Odmah** (zadano) - jedan e‑mail po čekajućem odobrenju, poslan čim je odobrenje kreirano.
- **Svakog sata** - jedan sažeti e‑mail po satu koji sumira sva odobrenja stavljena u red u tom satu.
- **Dnevno** - jedan sažeti e‑mail na svakih 24 sata.
- **Onemogućeno** - nema e‑mailova. Korisnik i dalje može pregledavati odobrenja putem UI‑ja ulaznog sandučića; samo ne prima obavijesti.

The user changes this setting on their own profile, not on the agent edit form. This is intentional - one tenant might have ten agents, and a moderator should not have to set their preferred frequency on every agent independently.

### Cron zadaci koji pokreću sažetke

- **`hourly-agent-approval-digest`** - pregledava svaki sat, grupira odobrenja stavljena u red od posljednjeg sažetka svakog korisnika, šalje jedan e‑mail po korisniku.
- **`daily-agent-approval-digest`** - isto, dnevno.
- **`agent-approval-reaper`** - uklanja odobrenja koja su starija od 90 dana, neovisno o stanju.

The hourly and daily digest crons are scoped per-recipient: a user with hourly frequency is processed by the hourly cron and skipped by the daily one (and vice versa). Immediate-frequency users are notified by the approval-create code path, not by the crons.

### Stanje deduplikacije

The platform tracks which users have already been emailed about each approval. Once a user has been notified (immediately or in a digest), they will not be emailed again for the same approval - even if they change their frequency from immediate to daily mid-cycle.

### Odobravanje iz e‑maila

Each notification email contains a one-click signed login link that takes the reviewer directly to the approval detail page, already authenticated. They can approve, reject, or open the [Refine Prompts](#refining-prompts) flow from there.

### Što ako ne postoje administratori

If `notifyMode` is `All admins and moderators` but the tenant has no super admins, comment moderator admins, or account owners with valid emails, the platform logs a warning and the approval still queues - just nobody gets notified about it. It will sit in the inbox until someone happens to look.

If `notifyMode` is `Specific users` but you have not selected any users, same outcome.

### Što ako su obavijesti o naplati onemogućene

[Budget Alerts](#budget-alerts) – e‑mailovi vezani uz proračun – idu administratore naplate **neovisno o postavkama obavijesti po korisniku**. To je namjerno: prekoračenja proračuna utječu na trošak, a vlasnik naplate mora biti obaviješten.

Approval notifications honor only the per-user agent-approval frequency setting. They do not check the broader admin-notifications opt-out - a user who has opted out of admin notifications will still receive approval emails if they are on the reviewer list, unless their agent-approval frequency is set to **Disabled**.

### Vidi također

- [Approval Workflow](#approval-workflow) za cijeli životni ciklus odobrenja.
- [Refining Prompts](#refining-prompts) za radni tok "Stalno odobravam istu vrstu greške".