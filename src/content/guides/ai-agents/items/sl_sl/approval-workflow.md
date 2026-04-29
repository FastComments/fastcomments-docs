**Odobritev** je klic orodja v čakalni vrsti, ki zahteva, da ga človek potrdi ali zavrne, preden ga platforma izvede.

### Nastavitev odobritev

Na obrazcu za urejanje agenta odsek **Odobritve** navaja vsa orodja, ki jih agent lahko pokliče (seznam dovoljenih), in vam omogoča, da označite tiste, ki jih mora pregledati človek. Neoznačena orodja se izvedejo takoj. Označena orodja se postavijo v čakalno vrsto.

Prepovedana orodja so *takoj zavrnjena*, niso postavljena v čakalno vrsto - odobritve veljajo samo za orodja, ki so sicer dovoljena.

### Kaj se zgodi, ko se sproži ukrep, ki zahteva odobritev

1. Agent izbere klic orodja (npr. `ban_user`) z argumenti, utemeljitvijo in stopnjo zaupanja.
2. Namesto izvedbe platforma postavi v čakalno vrsto odobritev v stanju `PENDING` z imenom orodja, argumenti, utemeljitvijo, stopnjo zaupanja in posnetkom konteksta, ki opisuje sprožilec, ki ga je ustvaril.
3. Opozorila se pošljejo recenzentom (glejte [Obvestila o odobritvah](#approval-notifications)).
4. Zagon agenta se dokonča in se zabeleži - dejanje je prikazano kot **Čaka na odobritev** v [Pogledu podrobnosti zagona](#run-detail-view).

### Pregled odobritev

Predal za odobritve na [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) navaja čakajoče, odobrene, zavrnjene in odobritve z neuspešno izvedbo. Za vsako:

- **Ime orodja in argumenti** - točno to, kar agent želi narediti.
- **Razmišljanje agenta** - utemeljitev, ki jo je agent podal.
- **Stopnja zaupanja** - agentova samoocenjena stopnja zaupanja, od 0.0 do 1.0.
- **Posnetek konteksta** - komentar, stran in uporabnik, na katerega je dejanje usmerjeno.

Dva gumba: **Sprejmi** in **Zavrni**. Sprejmi dejansko izvede orodje; Zavrni zavrže.

### Stanja odobritve

Odobritev prehaja skozi naslednja stanja:

| State | Meaning |
|---|---|
| `PENDING` | Čakanje na odločitev človeka. |
| `APPROVED` | Človek je odobril in dejanje se je izvedlo. |
| `REJECTED` | Človek je zavrnil. Dejanje se ni izvedlo. |
| `EXECUTION_FAILED` | Človek je odobril, vendar je izvedba spodletela (npr. ciljnega komentarja je že bil izbrisan). |
| `EXECUTING` | Prehodno stanje: človek je kliknil Sprejmi in dejanje se izvaja. Uporablja se za serializacijo sočasnih klikov Sprejmi, da orodje z dejanskimi stranskimi učinki nikoli ne teče dvakrat. |

Stanje `EXECUTING` je pomembno, ko dva recenzenta hkrati klikneta Sprejmi - en zmaga, drugi vidi, da se je odobritev že premaknila.

### Kaj se počisti

Čakajoče odobritve ostanejo v stanju čakanja, dokler ni sprejeta odločitev. Samodejno potečejo po **90 dneh** od ustvarjanja. Odobritve v katerem koli drugem stanju se prav tako počistijo po 90 dneh v interesu urejenosti shrambe.

Polja odobritve "odločil" / "odločeno ob" / "izvedeno ob" / "rezultat izvedbe" se zapolnijo, ko odobritev prehaja med stanji - vse je vidno v podrobnem pogledu predala.

### Integracija webhookov

Ob premikih odobritev sprožita dva dogodka webhookov:

- **`approval.requested`** - ob vnosu v stanje PENDING.
- **`approval.decided`** - ob prehodu v APPROVED, REJECTED ali EXECUTION_FAILED.

Oba sta podpisana kot vsi drugi webhooki. Glejte [Webhook Events](#webhook-events) in [Webhook Payloads](#webhook-payloads).

### Utrjevanje: preverjanje znanih orodij

Odobritve zavrnejo postavitev v čakalno vrsto kateregakoli imena orodja, ki ni prepoznano kot orodje agenta. To je preverjanje obrambne globine: tudi če bodo prihodnje poti kode prenesle ime orodja, izpeljano z LLM, v tok odobritve, ta niz ne more nikoli pristati kot postavljena odobritev. Nabor znanih imen orodij je enak naboru, navedenemu v [Tools Reference](#tools-overview).

### Pogosti vzorci omejevanja

- **Nov moderacijski agent** - omejite `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email`. Spremljajte predal nekaj tednov, nato odstranite omejitve pri orodjih z nizko napakovitostjo.
- **Dolgoročni moderacijski agent** - za vedno obdržite omejitve za `ban_user` in vsa neobrnljiva dejanja (`deleteAllUsersComments`, `banIP`).
- **Regija EU** - `ban_user` je zaklenjen zaradi člena 17 ne glede na to, kaj označite. Glejte [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Česa odobritve ne naredijo

- Ne zadržujejo drugih klicev orodij agenta. Zagon agenta lahko pokliče več orodij, in samo omejena so postavljena v čakalno vrsto - ostalo se izvede kot običajno.
- Ne prekličejo zagona agenta, če človek zavrne. Neomejeni del zagona je že izveden.