**Uporabljate Moodle?** Prav tako izdajamo namenski Moodle vtičnik za FastComments z bolj tesno integracijo kot LTI 1.3 (sprožilci sinhronizacije ocen, poglobljeno poročanje o dejavnostih, nativni uporabniški vmesnik za nastavitve Moodla). Oglejte si <a href="/guide-installation-moodle.html" target="_blank">navodila za namestitev Moodle vtičnika</a>. Spodnji postopek LTI 1.3 je prava izbira, če želite eno registracijo, ki pokriva tudi druge LMS-e, ali če vaš skrbnik Moodla ne bo namestil vtičnikov tretjih oseb.

Moodle 4.0+ podpira dinamično registracijo LTI 1.3 prek vtičnika External Tool.

#### Odprite zaslon za upravljanje orodij

1. Prijavite se v Moodle kot skrbnik spletnega mesta.
2. Pojdite na **Administracija spletnega mesta** > **Vtičniki** > **Moduli dejavnosti** > **External tool** > **Upravljaj orodja**.

#### Prilepite URL

Videli boste kartico z oznako **URL orodja**. Prilepite URL za registracijo FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">dobite ga tukaj</a>) v besedilno polje in kliknite **Add LTI Advantage**.

Moodle odpre zaslon za registracijo, ki prikazuje identiteto orodja in dovoljenja, ki jih zahteva. Preverite in kliknite **Aktiviraj** (ali **Registriraj**, odvisno od različice Moodla).

Pojavno okno se zapre, ko je registracija zaključena; novo orodje FastComments se prikaže na seznamu **Orodja** z statusom **Aktivno**.

#### Omogočite

Privzeto Moodle doda nova orodja na seznam "Course tools", vendar jih ne prikaže v izbirniku dejavnosti. Če želite FastComments omogočiti za celoten predmet:

1. Kliknite ikono zobnika na kartici FastComments.
2. Pod **Uporaba konfiguracije orodja**, izberite **Prikaži v izbirniku dejavnosti in kot vnaprej konfigurirano orodje**.
3. Shrani.

Učitelji lahko zdaj dodajo FastComments v kateri koli predmet preko **Dodaj dejavnost ali vir** > **FastComments**.