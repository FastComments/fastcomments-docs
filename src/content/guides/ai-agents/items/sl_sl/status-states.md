Agent ima enega od treh statusov:

### Disabled

Agent je izklopljen. Noben sprožilec se ne obdeluje in agent se ne pojavi v poti razporejevalnika. Njegova zgodovina zaganjanj, analitika in pomnilnik ostanejo - če ga kasneje ponovno omogočite, so zgodovinski podatki še vedno na voljo.

Use `Disabled` when:
- Želite odstraniti agenta iz rotacije, ne da bi ga izgubili.
- Agent se neobičajno vede in ga morate takoj ustaviti, medtem ko preiskujete.
- Agente sezonsko menjate (npr. gostitelj, ki dela samo med prazniki).

### Dry Run - privzeto za nove agente

Agent deluje od začetka do konca - obdeluje sprožilce, kliče LLM, izbere klice orodij, izračuna utemeljitve in stopnjo zaupanja - vendar **se ne izvede nobeno resnično dejanje**. Vsako zaganjanje je zabeleženo z značko **Dry Run** v [Zgodovina zaganjanj](#run-history).

Use `Dry Run` when:
- Nov agent je ravnokar iz škatle. Vsak začetni predlog je nastavljen v dry-run.
- Ste uredili poziv ali spremenili nabor sprožilcev in želite videti, kako se sprememba obnaša, preden jo potrdite.
- Izvajate [preizkus zagona / ponovitev](#test-runs-replays) (ponovitve prisilijo dry-run ne glede na status agenta).

Platforma zaračuna tokene za dry-run zagone - klic LLM se še vedno zgodi, le stranski učinki so preskočeni. Omejitve proračuna veljajo tudi za dry-run. Oglejte si [Pregled proračunov](#budgets-overview).

### Enabled

Agent izvaja dejanska dejanja. Klici orodij se izvršijo - ali pa se uvrstijo v čakalno vrsto za [odobritev](#approval-workflow), če je dejanje zadržano za odobritev.

Use `Enabled` after dry-run output looks correct.

### Menjava statusa

Na obrazcu za urejanje lahko preklapljate med katerima koli dvema statusoma. Preklop iz Dry Run v Enabled ne ponovi izvajanja dry-run dejanj retroaktivno - ta ostanejo kot zgodovina dry-run. Novi sprožilci od tega trenutka dalje se izvajajo v živo.

Preklop iz Enabled v Disabled med izvajanjem ne prekliče tekočega zagona. Trenutno izvajajoči se sprožilec se dokonča (z vsem, kar je že začel); naslednji sprožilec je opuščen, ker je agent sedaj Disabled.

### Status med težavami z obračunom

Če postane obračun pri vašem najemniku neveljaven, so vsi agenti dejansko začasno ustavljeni ne glede na shranjeni status - sprožilci so zavrnjeni z `BILLING_INVALID` dokler obračun ni obnovljen. Polje shranjenega statusa se ne spremeni; razporejevalnik preprosto noče izvajati. Oglejte si [Načrti in upravičenost](#plans-and-eligibility).