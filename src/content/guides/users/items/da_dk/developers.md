For Developers whom you may not want to be `Administrators`, consider creating an `Administrator` user
with the following permissions:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

This set of permissions will give a developer everything they need to setup FastComments as well
as the visibility into the system required to ensure it is working.

The reasoning for these permissions is as follows:

1. **Analytics Admin**: Dette kan bruges til at fejlfinde brugen af FastComments.
2. **Customizations Admin**: Dette vil være nødvendigt for at anvende brugerdefineret styling på kommentar-widgeten.
3. **Data Management Admin**: Dette vil være nødvendigt for at administrere importer og eksporter samt opsætte webhooks.
4. **Comment Moderation Admin**: Dette vil være nødvendigt for at se kommentardata, i det mindste under opsætningen.
5. **API/SSO Admin**: Dette vil give dem mulighed for at hente API keys direkte fra vores platform. Vi anser dette som mere sikkert end en `Administrator` kopierer den for dem og sender API Secret via e-mail, hvilket måske ikke er særlig sikkert.