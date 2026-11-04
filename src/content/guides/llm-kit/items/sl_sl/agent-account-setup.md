Kodirni agent, kot so Claude Code, Cursor ali pomočnik na osnovi MCP, lahko nastavi FastComments namesto vas, brez da bi izpolnili prijavni obrazec. To je uporabno, ko agenta prosite, da "doda komentarje moji spletni strani", vi pa še nimate računa.

### Kako deluje

1. Agent ustvari nov račun in prejme API ključ ter povezavo za prevzem računa. API ključ deluje takoj, zato lahko agent konfigurira račun in namesti pripomoček (widget) na vašo spletno stran.
2. Agent vam posreduje povezavo za prevzem. Odprite jo v brskalniku, se prijavite ali ustvarite prijavo in potrdite prevzem. Račun je nato vaš: upravljate ga, njegovo obračunavanje in njegove API ključe iz nadzorne plošče. Stran navaja API ključ, ki ga ima agent, tako da ga lahko prekličete, če ne želite več, da ima agent ali tisti, ki ga upravlja, dostop.
3. Če nihče ne odpre povezave za prevzem v 72 urah, sta račun in njegov ključ izbrisana. Prosite agenta, da ustvari novega.

Dokler račun ni prevzet, ima enake omejitve kot običajno brezplačno preizkusno obdobje.

### Če že imate račun

Vsaka prijava ima en račun. Če odprete povezavo za prevzem, medtem ko ste prijavljeni v obstoječi račun, vam stran omogoča izbiro:

- **Pripni k mojemu računu** naredi nov račun upravljani najemnik (tenant) računa, v katerega ste prijavljeni. To zahteva plačljivi paket z belo označbo (white labeling), uporaba novega najemnika pa se obračuna na vaš račun.
- **Odjavi se in prevzemi z drugo prijavo** vas odjavi in vrne na stran za prevzem, da ga lahko prevzamete z drugo prijavo.

### Za avtorje agentov

Navodila za agente na [fastcomments.com/agents.md](https://fastcomments.com/agents.md) opisujejo klic za ustvarjanje računa, polja v odgovoru in kako posredovati povezavo za prevzem osebi, za katero delate. Klic ne potrebuje API ključa in je omejen na hitrost glede na IP naslov.