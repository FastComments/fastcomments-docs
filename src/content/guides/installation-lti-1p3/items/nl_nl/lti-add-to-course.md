Zodra FastComments bij uw LMS is geregistreerd, voegen instructeurs het aan cursussen toe op dezelfde manier als elke andere LTI externe tool.

#### D2L Brightspace

In het inhoudsgebied van een cursus:

1. Klik **Bestaande activiteiten toevoegen** > **Externe leertools**.
2. Kies **FastComments** uit de lijst.
3. De tool verschijnt als een topic in het inhoudsgebied. Open het één keer om de discussiedraad voor die bron te initialiseren.

#### Moodle

In een cursus:

1. Zet **Bewerkmodus** aan.
2. Klik in de sectie waar u opmerkingen wilt op **Voeg een activiteit of bron toe**.
3. Kies **FastComments** in de activiteitskiezer.
4. Sla op. Studenten zien de discussiedraad ingebed in de sectie.

#### Blackboard Learn

In een cursus:

1. Navigeer naar een inhoudsgebied.
2. Klik **Inhoud maken** > **FastComments** (onder "Leertools").
3. Configureer een naam en verstuur.

#### Sakai

Sitebeheerders voegen de tool toe via **Site Info** > **Tools beheren** > scroll naar **Externe Tools** > selecteer **FastComments** > **Doorgaan**.

#### How Threads Are Scoped

FastComments maakt een aparte discussiedraad per **(LMS-instantie, cursus, resource-link)**. Dat betekent:

- Twee verschillende cursussen in hetzelfde LMS krijgen afzonderlijke draden, zelfs als ze dezelfde toolnaam gebruiken.
- Dezelfde FastComments-tool die op twee plaatsen binnen één cursus wordt gebruikt, creëert twee draden.
- Twee verschillende Brightspace-installaties onder hetzelfde FastComments-account krijgen afzonderlijke draden - de LMS-hostnaam maakt deel uit van de identificatie van de draad.

#### SSO

Studenten zien geen inlogscherm. Het LMS stuurt hun identiteit (naam, e-mail, avatar, rol) naar FastComments via de LTI-launch, en FastComments logt hen automatisch in. Hun opmerkingen worden toegeschreven aan hun LMS-account.

Gebruikers met de LMS-rollen **Instructor** of **Administrator** worden automatisch toegewezen aan FastComments moderator-/admin-rollen voor de draad.