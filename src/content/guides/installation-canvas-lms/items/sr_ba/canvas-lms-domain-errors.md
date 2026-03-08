Možda ćete dobiti grešku autorizacije, poput sljedeće:

<div class="screenshot white-bg">
    <div class="title">Nedostaje konfiguracija domene</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-webflow-step-5.png" alt="Nedostaje konfiguracija domene" />
</div>

Do toga dolazi zato što FastComments ne zna da treba dozvoliti upotrebu vašeg naloga na ovoj domeni.

U tom slučaju, rješenje je jednostavno. Samo trebate dodati domenu vaše Canvas instance na vaš FastComments nalog.

<a href="https://fastcomments.com/auth/my-account/configure-domains" target="_blank">Idite ovdje da dodate svoju stranicu na svoj nalog.</a>

Dodajte domenu vaše Canvas instance (npr. `yourschool.instructure.com`) i komentari bi se trebali učitati ispravno.