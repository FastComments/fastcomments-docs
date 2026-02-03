FastComments samodejno beleži podrobne dogodke za vsak komentar, da zagotovi preglednost pri odločitvah moderiranja in sistemskih dejanjih. Ti zapisi pomagajo razumeti, zakaj je bil komentar odobren, označen kot spam ali mu je bil spremenjen status.

## Dostop do zapisov komentarjev

Za ogled zapisov za določen komentar:

1. Pojdite na stran **Moderate Comments** v vašem FastComments nadzornem panelu
2. Najdite komentar, ki ga želite pregledati
3. Kliknite gumb **View Logs** (ikona ure) v vrstici z dejanji komentarja
4. Prikaže se pogovorno okno, ki prikazuje celotno zgodovino dogodkov za ta komentar

Vsak vnos v zapisu prikazuje:
- **Kdaj** - Časovni žig dogodka
- **Kdo** - Uporabnik ali sistem, ki je sprožil dogodek (če je ustrezno)
- **Kaj** - Vrsta dejanja ali dogodka
- **Podrobnosti** - Dodatni kontekst, kot so vrednosti pred/po, imena engine-ov ali povezani podatki

## Dogodki v zapisu komentarjev

Vsak komentar ima zapisan dnevnik dogodkov, ki se zgodijo v njegovem življenjskem ciklu. Spodaj so tipi dogodkov, ki se beležijo:

### Dogodki anonimizacije
- **Anonymized** - Vsebina komentarja je bila izbrisana in uporabnik označen kot izbrisan
- **RestoredFromAnonymized** - Komentar je bil obnovljen iz anonimiziranega stanja

### Dogodki odobritve
- **ApprovedDueToPastComment** - Komentar odobren, ker ima uporabnik prej odobrene komentarje (vključuje sklic na prejšnji komentar)
- **ApprovedIsAdmin** - Komentar odobren, ker je uporabnik admin
- **NotApprovedRequiresApproval** - Komentar zahteva ročno odobritev
- **NotApprovedLowTrustFactor** - Komentar ni bil odobren zaradi nizkega faktorja zaupanja uporabnika (vključuje vrednost faktorja zaupanja)

### Dogodki odobritve komentarjev na profilih

Ti dogodki se nanašajo posebej na komentarje na uporabniških profilih:

- **ApprovedProfileAutoApproveAll** - Komentar na profilu samodejno odobren, ker je lastnik profila omogočil avtomatsko odobritev vseh komentarjev
- **ApprovedProfileTrusted** - Komentar na profilu odobren, ker je komentator zaupanja vreden (vključuje sklic na komentar, ki je vzpostavil zaupanje)
- **NotApprovedProfileManualApproveAll** - Komentar na profilu zahteva ročno odobritev, ker je lastnik profila omogočil ročno odobritev
- **NotApprovedProfileNotTrusted** - Komentar na profilu ni bil odobren, ker komentator ni zaupanja vreden
- **NotApprovedProfileNewUser** - Komentar na profilu ni bil odobren, ker je komentator nov uporabnik

### Dogodki zaznavanja spama
- **IsSpam** - Komentar označen kot spam s strani detekcijskega engine-a (vključuje kateri engine je sprejel odločitev)
- **IsSpamDueToBadWords** - Komentar označen kot spam zaradi filtra za neprimerne besede
- **IsSpamFromLLM** - Komentar označen kot spam s strani AI/LLM engine-a (vključuje ime engine-a, odgovor in št. tokenov)
- **IsSpamRepeatComment** - Komentar označen kot spam zaradi ponavljanja (vključuje kateri engine ga je zaznal)
- **NotSpamIsOnlyImage** - Komentar ni bil označen kot spam, ker vsebuje samo slike
- **NotSpamIsOnlyReacts** - Komentar ni bil označen kot spam, ker vsebuje samo reakcije
- **NotSpamNoLinkOrMention** - Komentar ni bil označen kot spam, ker ne vsebuje sumljivih povezav ali omemb
- **NotSpamPerfectTrustFactor** - Komentar ni bil označen kot spam zaradi visokega faktorja zaupanja uporabnika
- **NotSpamTooShort** - Komentar ni bil označen kot spam, ker je prekratek za analizo
- **NotSpamSkipped** - Preverjanje spama je bilo preskočeno
- **NotSpamFromEngine** - Komentar ocenjen kot ne-spam s strani detekcijskega engine-a (vključuje ime engine-a in faktor zaupanja)

### Dogodki neprimernih besed/profanosti
- **BadWordsCheckFailed** - Preverjanje filtra za neprimerne besede je naletelo na napako
- **BadWordsFoundBadPhrase** - Filter je zaznal neprimerno frazo (vključuje frazo)
- **BadWordsFoundBadWord** - Filter je zaznal neprimerno besedo (vključuje besedo)
- **BadWordsNoDefinitionForLocale** - Za jezik komentarja ni na voljo definicij za filter (vključuje lokalno nastavitev)

### Dogodki preverjanja uporabnika
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Komentar zahteva preverjanje, vendar uporabnik ni v preverjeni seji
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Komentar zahteva preverjanje, vendar uporabnik še ni preverjen
- **InVerifiedSession** - Uporabnik, ki objavlja komentar, je v preverjeni seji
- **SentVerificationEmailNoSession** - Potrditveno e-poštno sporočilo poslano nepreverjenemu uporabniku
- **SentWelcomeEmail** - Pozdravno e-poštno sporočilo poslano novemu uporabniku

### Dogodki zaupanja in varnosti
- **TrustFactorChanged** - Faktor zaupanja uporabnika je bil spremenjen (vključuje vrednosti pred in po)
- **SpamFilterDisabledBecauseAdmin** - Filtriranje spama je bilo zaobideno za administratorskega uporabnika
- **TenantSpamFilterDisabled** - Filtriranje spama onemogočeno za celoten tenant
- **RepeatCommentCheckIgnored** - Preverjanje ponavljajočih se komentarjev je bilo izpuščeno (vključuje razlog)
- **UserIsAdmin** - Uporabnik identificiran kot admin
- **UserIsAdminParentTenant** - Uporabnik identificiran kot admin starševskega tenanta
- **UserIsAdminViaSSO** - Uporabnik identificiran kot admin preko SSO
- **UserIsMod** - Uporabnik identificiran kot moderator

### Spremembe statusa komentarja

Dogodki spremembe statusa vsebujejo vrednosti pred in po, ter uporabnika, ki je izvedel spremembo:

- **ExpireStatusChanged** - Status poteka komentarja je bil spremenjen
- **ReviewStatusChanged** - Status pregleda komentarja je bil spremenjen
- **SpamStatusChanged** - Status spama komentarja je bil posodobljen
- **ApproveStatusChanged** - Status odobritve komentarja je bil spremenjen
- **TextChanged** - Besedilo komentarja je bilo urejeno (vključuje besedilo pred in po)
- **VotesChanged** - Štetje glasov za komentar je bilo posodobljeno (vključuje podroben razčlen glasov)
- **Flagged** - Komentar je bil prijavljen s strani uporabnikov
- **UnFlagged** - Prijave komentarja so bile odstranjene

### Moderacijska dejanja
- **Pinned** - Komentar je moderator pripenil (vključuje kdo ga je pripenil)
- **UnPinned** - Komentar je moderator odpenil (vključuje kdo ga je odpenil)

### Dogodki obveščanja
- **CreatedNotifications** - Za komentar so bila ustvarjena obvestila (vključuje število obvestil)
- **NotificationCreateFailure** - Ustvarjanje obvestil ni uspelo
- **BadgeAwarded** - Uporabniku je bila podeljena značka za komentar (vključuje ime značke)

### Dogodki objavljanja
- **PublishedLive** - Komentar je bil objavljen za žive naročnike (vključuje število naročnikov)

### Integracijski dogodki
- **WebhookSynced** - Komentar je bil sinhroniziran preko webhook-a

### Dogodki pravil za spam
- **SpamRuleMatch** - Komentar ustreza po meri določenemu pravilu za spam (vključuje podrobnosti pravila)

### Lokalizacijski dogodki
- **LocaleDetectedFromText** - Jezikovna lokalizacija je bila samodejno zaznana iz besedila komentarja (vključuje zaznani jezik in lokalno nastavitev)

## Primeri uporabe zapisov komentarjev

Zapisi komentarjev se samodejno ustvarijo in shranijo z vsakim komentarjem. Ponujajo dragocene vpoglede za:

- **Razumevanje odločitev moderiranja** - Oglejte si natančno, zakaj je bil komentar odobren, zadržan v pregledu ali označen kot spam
- **Razhroščevanje težav z odobritvijo/spamom** - Sledite odločitveni logiki, ko komentarji ne delujejo, kot se pričakuje
- **Spremljanje vzorcev vedenja uporabnikov** - Spremljajte spremembe faktorja zaupanja in status preverjanja
- **Revidiranje dejanj moderatorjev** - Preglejte, katera dejanja so moderatorji izvedli za določene komentarje
- **Preiskovanje učinkovitosti filtra spama** - Oglejte si, kateri detekcijski engine-i ujemajo spam in kateri ne
- **Odpravljanje težav z integracijami** - Preverite sinhronizacije webhook-ov in dostavo obvestil

Ti zapisi pomagajo ohranjati preglednost v procesu moderiranja in pomagajo pri natančnem nastavljanju vedenja sistema komentarjev.