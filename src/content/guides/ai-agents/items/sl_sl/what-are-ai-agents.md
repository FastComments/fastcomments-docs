**AI Agent** je avtonomni delavec, omejen na vaš FastComments najem, ki spremlja dogodke v vaši skupnosti in ukrepa v vašem imenu.

Vsak agent ima tri stvari, nad katerimi imate nadzor:

1. **Osebnost.** Besedilni začetni poziv, ki določa ton, vlogo in slog odločanje ("Vi ste topel pozdravitelj skupnosti", "Uveljavljate pravila skupnosti, vendar raje opozarjate kot blokirate" ipd.).
2. **Ena ali več sprožilcev.** Seznam dogodkov, ki agenta prebudijo - nov komentar, komentar, ki preseže prag glasov ali prijav, moderatorjeva akcija, prv komentar uporabnika na strani in drugi. Celoten seznam je v [Pregled sprožilcev dogodkov](#triggers-overview).
3. **Seznam dovoljenih orodij.** Kaj agentu dovoljujete, da počne - objavi komentar, oceni (vote), pripne, zaklene, označi kot neželeno pošto, blokira uporabnika, pošlje opozorilo prek zasebnega sporočila, podeli značko, pošlje e-pošto, shrani in poišče deljeno pomnjenje. Celoten seznam je v [Pregled dovoljenih orodij](#tools-overview).

Ko se sprožilec aktivira, agent prejme kontekstno sporočilo, ki opisuje, kaj se je zgodilo (komentar, stran, dodatni kontekst niti/uporabnika/strani) in mu je predložen njegov začetni poziv ter vaša pravila skupnosti. Nato kliče orodja za ukrepanje in pri vsakem klicu zabeleži utemeljitev in oceno zaupanja.

### Agenti delujejo asinhrono

Agenti **nikoli ne blokirajo uporabniškega dejanja, ki jih je sprožilo**. Bralec objavi komentar, komentar se shrani in prikaže v niti, odgovor je vrnjen, šele nato agent deluje na njem - bodisi takoj ali po nastavljenem zamiku (glejte [Deferred Triggers](#trigger-deferred-delay)). Nič, kar agent naredi, ne poveča latence uporabniške izkušnje.

### Zakaj jih uporabljati

- **Moderirajte v veliki meri.** Označite očitno neželeno pošto in blokirajte ponavljajoče kršitelje brez stalnega nadzora čakalne vrste.
- **Pozdravljajte nove komentatorje.** Odgovorite komentatorjem, ki pišejo prvič, v vašem tonu.
- **Izpostavite najboljšo vsebino.** Pripnite pomembne vrhnje komentarje, ko presežejo prag glasov.
- **Dosledno uveljavljajte pravila.** Nežno izvedite isto politiko pri vsakem mejni komentarju.
- **Povzemite dolge niti.** Objavite nevtralne povzetke večstranskih razprav.

### Kaj vam daje nadzor

- **Način Dry-run.** Vsak nov agent je privzeto v **Dry Run** načinu: obdela sprožilce, zažene model in zabeleži, kaj bi naredil, vendar ne izvede nobenih dejanskih ukrepov. Glejte [Način Dry-Run](#dry-run-mode).
- **Odobritve.** Poljubna podmnožica akcij je lahko vezana na ročno odobritev. Glejte [Potek odobritve](#approval-workflow).
- **Proračuni po agentu in po računu.** Strogi dnevni in mesečni limiti. Glejte [Pregled proračunov](#budgets-overview).
- **Seznam dovoljenih orodij.** Prepovedana orodja so odstranjena iz nabora modela - agent jih preprosto ne more zahtevati. Glejte [Pregled dovoljenih orodij](#tools-overview).
- **Revizijska polja pri vsakem ukrepu.** Model mora vključiti utemeljitev in oceno zaupanja. Obe se prikažeta v časovnici izvajanja in pri vsaki odobritvi. Glejte [Pogled podrobnosti izvajanja](#run-detail-view).
- **17. člen EU DSA.** V regiji EU so popolnoma avtomatizirane blokade onemogočene. Glejte [Usklajenost z 17. členom EU DSA](#eu-dsa-compliance).
- **Brez treniranja na vaših podatkih.** FastComments uporablja ponudnike, ki ne trenirajo na vaših pozivih ali komentarjih.

### Kje se umeščajo skupaj s človeško moderacijo

Agenti in človeški moderatorji uporabljajo isto platformo za komentarje: agenti izvajajo ukrepe prek istih kanalov (odobri, neželena pošta, blokiraj, značka, pripni, zakleni, piši) in ti ukrepi se pojavijo v istih [Zapisih komentarjev](/guide-moderation.html#comment-logs), na isti [Strani za moderacijo](/guide-moderation.html#moderate-comments-page) in v istih tokovih obvestil. Agenti in ljudje vidijo delo drug drugega in lahko medsebojno reagirajo - moderatorjeve akcije so same po sebi veljavni sprožilci za agente (glejte [Sprožilec: moderator je pregledal komentar](#trigger-moderator-reviewed) in sorodne).