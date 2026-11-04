**Bruger du Moodle?** Vi udgiver også et dedikeret Moodle-plugin til FastComments med en tættere integration end LTI 1.3 (karaktersynkroniserings-hooks, dybere aktivitetsrapportering, indbygget Moodle-indstillingsgrænseflade). Se <a href="/guide-installation-moodle.html" target="_blank">installationsguiden til Moodle-pluginet</a>. LTI 1.3-flowet nedenfor er det rigtige valg, hvis du ønsker en enkelt registrering, der også dækker andre LMS'er, eller hvis din Moodle-administrator ikke installerer tredjeparts-plugins.

Moodle 4.0+ understøtter LTI 1.3 Dynamic Registration via External Tool-pluginet.

#### Åbn værktøjsadministrationsskærmen

1. Log ind på Moodle som administrator.
2. Naviger til **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools**.

#### Indsæt URL'en

Du vil se et kort mærket **Tool URL**. Indsæt FastComments-registrerings-URL'en (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">få den her</a>) i tekstfeltet, og klik på **Add LTI Advantage**.

Moodle åbner en registreringsskærm, der viser værktøjets identitet og de tilladelser, det anmoder om. Gennemgå og klik på **Activate** (eller **Register**, afhængigt af Moodle-versionen).

Popup-vinduet lukker, når registreringen er fuldført; det nye FastComments-værktøj vises på **Tools**-listen med status **Active**.

#### Gør det tilgængeligt

Som standard tilføjer Moodle nye værktøjer til "Course tools"-listen, men viser dem ikke i aktivitetsvælgeren. For at gøre FastComments tilgængeligt på tværs af kurset:

1. Klik på tandhjulsikonet på FastComments-flisen.
2. Under **Tool configuration usage** skal du vælge **Show in activity chooser and as a preconfigured tool**.
3. Gem.

Undervisere kan nu tilføje FastComments til ethvert kursus via **Add an activity or resource** > **FastComments**.