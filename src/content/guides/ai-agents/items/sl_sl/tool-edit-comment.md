Orodje Edit omogoča agentu zamenjavo besedila obstoječega komentarja. Je uničujoče na način, kot večina drugih orodij za moderiranje ni: prepiše vsebino, ki jo je ustvaril uporabnik. Rezervirajte ga za ozke, jasne primere.

### Kaj naredi

Agent posreduje ID komentarja in nadomestno telo. Platforma zapiše novo besedilo v komentar in zabeleži vnos `TextChanged` v revizijski dnevnik komentarja, ki zajame tako staro besedilo kot novo besedilo. Izvirnik nikoli ni izgubljen — moderatorji lahko preberejo, kaj je komentar vseboval pred ureditvijo agenta.

Zamenjava gre skozi enak proces upodabljanja kot človeška ureditev: maskiranje psovk, razčlenjevanje omemb, izluščanje hashtagov in obravnava povezav/slik se obnašajo enako, kot če bi izvirni avtor oddal novo besedilo.

### Obseg

Kot vsako orodje, ki spreminja komentarje, je Edit omejen na dovolilni seznam sprožilca — agent lahko uredi samo komentar, na katerem je sprožilec deloval, njegovega nadrejenega ali drug komentar v dosegu istega konteksta sprožilca. Poskus vbrizgavanja v poziv (prompt-injection) z zahtevo "edit comment XYZ", kjer je XYZ nepovezan, bo zavrnjen na strežniški strani še preden se izvršitelj zažene.

### Zanke

Ko agent uredi komentar, platforma sproži sprožilec `COMMENT_EDIT`, kot bi se zgodilo pri človeški ureditvi, vendar **onemogoči pošiljanje drugim agentom**. To prepreči, da bi se dva agenta, ki oba poslušata `COMMENT_EDIT`, medsebojno izmenjevala urejanja.

### Kdaj dovoliti

Za agente, ki urejajo PII (odstranjevanje osebnih podatkov), ali za agente za samourejanje povzetkov/digestov. Večina moderacijskih agentov tega orodja **ne** potrebuje — mark-spam, warn in ban pokrivajo tipično življenjsko pot.

### Odobritve

**Močno razmislite o omejevanju z odobritvijo**, še posebej med gradnjo zaupanja v agenta. Agent, ki prepisuje uporabnikove besede, je dejanje, ki ga bo skupnost opazila in nanj reagirala, in ga je težje "preklicati" z vidika ugleda kot izbris.

### Glej tudi

- [Trigger: Comment Edited](#trigger-comment-edit) - sprožilec, ki se sproži, ko se besedilo komentarja spremeni.
- [Approval Workflow](#approval-workflow) - kako omejiti orodje z ročnim pregledom.