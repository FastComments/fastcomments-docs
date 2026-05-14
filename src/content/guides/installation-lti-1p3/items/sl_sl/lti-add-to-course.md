Ko je FastComments registriran v vašem LMS, inštruktorji ga dodajo v predmete na enak način kot katero koli drugo zunanje LTI-orodje.

#### D2L Brightspace

V območju vsebine predmeta:

1. Kliknite **Dodaj obstoječe dejavnosti** > **Zunanja orodja za učenje**.
2. Izberite **FastComments** s seznama.
3. Orodje se prikaže kot tema v območju vsebine. Odprite ga enkrat, da inicializirate nit komentarjev za ta vir.

#### Moodle

V predmetu:

1. Vklopite **Način urejanja**.
2. V razdelku, kjer želite komentarje, kliknite **Dodaj dejavnost ali vir**.
3. Izberite **FastComments** v izbirniku dejavnosti.
4. Shrani. Študenti vidijo nit komentarjev vdelano v ta razdelek.

#### Blackboard Learn

V predmetu:

1. Pojdite v območje vsebine.
2. Kliknite **Ustvari vsebino** > **FastComments** (pod "Orodja za učenje").
3. Določite ime in potrdite.

#### Sakai

Vzdrževalci strani dodajo orodje prek **Site Info** > **Manage Tools** > pomaknite se do **External Tools** > izberite **FastComments** > **Continue**.

#### How Threads Are Scoped

FastComments ustvari ločeno nit komentarjev za vsak **(instanco LMS, predmet, povezava do vira)**. To pomeni:

- Dva različna predmeta v istem LMS prejmeta ločeni niti, tudi če uporabljata enako ime orodja.
- Isto orodje FastComments, uporabljeno na dveh mestih znotraj enega predmeta, ustvari dve niti.
- Dve različni namestitvi Brightspace pod istim računom FastComments prejmeta ločeni niti - gostiteljsko ime LMS je del identifikatorja niti.

#### SSO

Študenti ne vidijo zaslona za prijavo. LMS pošlje njihovo identiteto (ime, e-pošta, avatar, vloga) FastComments prek LTI-zagon, FastComments pa jih samodejno prijavi. Njihovi komentarji so pripisani njihovemu računu v LMS.

Uporabniki z vlogami LMS **Instructor** ali **Administrator** so samodejno preslikani na FastComments vloge moderatorja/administratorja za to nit.