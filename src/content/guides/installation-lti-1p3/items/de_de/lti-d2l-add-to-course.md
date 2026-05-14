Diese Seite beschreibt das Hinzufügen von FastComments zu einem Brightspace-Kurs, nachdem ein Administrator das Tool registriert und eine Bereitstellung erstellt hat. Wenn das Tool noch nicht registriert ist, lesen Sie zuerst die D2L-Registrierungsanleitung.

<div class="screenshot white-bg">
    <div class="title">FastComments eingebettet als Themen-Einheit in Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments running inside a Brightspace unit, showing threaded comments and an @-mention picker" />
</div>

Brightspace liefert zwei Inhaltsauthoring-Erlebnisse: **Classic Content** und das **New Content Experience** (auch **Lessons** genannt). Beide stellen FastComments bereit, aber die Menüpfade unterscheiden sich. Jeder Abschnitt unten behandelt beide Fälle, wo sie auseinandergehen.

#### FastComments-Tool finden

Das FastComments-Tool erscheint an zwei Stellen im Kursinhalt-Editor:

1. Im Aktivitätspicker, erreichbar über die **Add Existing**-Schaltfläche eines Moduls/einer Einheit (in älteren Brightspace-Versionen mit der Bezeichnung **Add Existing Activities**). In aktuellen Brightspace-Versionen zeigt der Picker FastComments direkt an; in älteren Versionen ist es unter einem Untermenü **External Learning Tools** verschachtelt. Beide Wege fügen FastComments als eigenständiges Thema hinzu.
2. Im **Insert Stuff**-Dialog des HTML-Editors, unter **LTI Advantage**. Dies bettet FastComments inline in ein HTML-Thema über den LTI-Deep-Linking-Flow ein.

Wenn FastComments in keinem der Picker erscheint, ist die Bereitstellung für die Organisationseinheit, die den Kurs enthält, nicht aktiviert. Bitten Sie Ihren Brightspace-Administrator, **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments** zu öffnen, die Bereitstellung zu öffnen und die Organisationseinheit des Kurses (oder eine übergeordnete Organisationseinheit) unter **Org Units** hinzuzufügen.

#### FastComments als Thema in einem Modul hinzufügen

Classic Content:

1. Öffnen Sie den Kurs und klicken Sie in der Navigationsleiste auf **Content**.
2. Wählen Sie das Modul aus, das die Diskussion enthalten soll (oder erstellen Sie eines über **Add a module**).
3. Klicken Sie auf **Add Existing** (älteres Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. Klicken Sie im Picker auf **FastComments**. Brightspace erstellt ein Thema im Modul und bringt Sie zurück zur Inhaltsansicht.
5. Klicken Sie auf das neue Thema. Benennen Sie es mit dem Inline-Titel-Editor in etwas Beschreibendes um, z. B. `FastComments Discussion`.

New Content Experience (Lessons):

1. Öffnen Sie den Kurs und klicken Sie auf **Content**.
2. Öffnen Sie die Einheit und die Lesson, die die Diskussion enthalten soll.
3. Klicken Sie auf **Add** > **Existing Activity** und wählen Sie **FastComments** (älteres Brightspace: verschachtelt unter **External Learning Tools**).
4. Die Aktivität wird zur Lesson hinzugefügt.
5. Klicken Sie auf den Aktivitätstitel, um ihn umzubenennen.

Beim ersten Öffnen des Themas durch einen beliebigen Benutzer (Dozent oder Student) initialisiert FastComments den Thread für diesen Resource Link. Der Thread ist an die Resource Link ID gebunden, sodass Umbenennen oder Verschieben des Themas nicht ändert, welcher Thread geladen wird.

#### FastComments inline in einem HTML-Thema einbetten

Verwenden Sie diesen Ablauf, wenn Kommentare unter einer Lektüre, einem Video oder anderem Inhalt auf derselben Themenseite erscheinen sollen, anstatt als separates Thema.

1. Öffnen oder erstellen Sie ein HTML-Thema im Modul/der Lesson.
2. Klicken Sie auf **Edit HTML**, um den Brightspace-HTML-Editor zu öffnen.
3. Setzen Sie den Cursor an die Stelle, an der der Kommentarthread erscheinen soll.
4. Klicken Sie auf die Schaltfläche **Insert Stuff** (Puzzle-Symbol in der Editor-Symbolleiste).
5. Scrollen Sie im Insert Stuff-Dialog zu **LTI Advantage** und klicken Sie auf **FastComments**.
6. FastComments öffnet einen Deep-Linking-Picker. Bestätigen Sie die Platzierung (die Standardoptionen funktionieren für Inhaltsdiskussionen); klicken Sie auf **Insert** oder **Continue**.
7. Brightspace kehrt mit einem Platzhalterblock, der den LTI-Launch darstellt, zum HTML-Editor zurück. Klicken Sie auf **Save and Close** im Thema.

Wenn das Thema geladen wird, ersetzt Brightspace den Platzhalter durch ein iframe, das FastComments per LTI automatisch startet. Studierende sehen den Diskussions-Thread inline.

Ein einzelnes HTML-Thema kann mehrere Deep-Linked FastComments-Einbettungen enthalten. Jede Einbettung erhält ihren eigenen Thread, weil jeder Deep Link eine eigene Resource Link ID erzeugt.

#### Modulthema vs. Inline-Quicklink

Wählen Sie den Ansatz mit dem **Modulthema**, wenn:

- Die Diskussion die primäre Aktivität für diesen Schritt im Modul ist.
- Sie möchten, dass das Thema im Inhaltsverzeichnis von Brightspace, in der Abschlussverfolgung und in Class Progress erscheint.

Wählen Sie den **Inline-Einbettung**-Ansatz, wenn:

- Kommentare unter anderem Inhalt auf derselben Seite stehen sollen.
- Sie keinen separaten, nachverfolgbaren Eintrag im Inhaltsverzeichnis wünschen.

#### Sichtbarkeit, Entwurf und Freigabebedingungen

Ein neues FastComments-Thema ist standardmäßig für Studierende sichtbar. Um es während der Einrichtung zu verbergen:

1. Klicken Sie im Inhaltseditor auf den Thementitel (Classic) oder das Drei-Punkte-Menü der Aktivität (New Content Experience).
2. Setzen Sie den Status auf **Draft** (Classic) oder schalten Sie die **Visibility** (New Content Experience) aus.

Entwurfs-Themen sind für Studierende unsichtbar. Lehrende und TAs sehen sie weiterhin mit einem „Draft“-Badge.

Um das Thema auf eine bestimmte Gruppe oder Sektion zu beschränken:

1. Öffnen Sie das Thema.
2. Klicken Sie im Thementitel-Menü auf > **Edit Properties In-place** (Classic) oder **Edit** > **Restrictions** (New Content Experience).
3. Unter **Release Conditions** klicken Sie auf **Create**.
4. Wählen Sie **Group enrollment** oder **Section enrollment**, wählen Sie die Gruppe/Sektion aus und speichern Sie.

Freigabebedingungen werden mit der rollenbasierten Zuordnung von FastComments kombiniert. Studierende, die das Thema nicht sehen können, erhalten keinen LTI-Start.

#### Was Studierende beim ersten Start sehen

Wenn ein Studierender das Thema anklickt (oder ein HTML-Thema mit einer Einbettung lädt):

1. Führt Brightspace den LTI 1.3-Launch im Hintergrund aus.
2. Erhält FastComments den Namen des Studierenden, die E-Mail, die Avatar-URL und die LMS-Rolle und meldet ihn automatisch an. Es erscheint keine FastComments-Anmeldemaske.
3. Wird der Kommentarthread für diesen Resource Link im Brightspace-iframe dargestellt.

Rollen-Abbildung beim Launch:

- Brightspace `Administrator` wird für den Thread zu einem FastComments **admin** (vollständige Moderation, Löschen, Sperren und Konfigurationszugriff).
- Brightspace `Instructor` wird zu einem FastComments **moderator** (anpinnen, verbergen, löschen, sperren).
- Alle anderen Rollen (`Learner`, `TeachingAssistant`, etc.) werden zu normalen Kommentierenden.

Kommentare werden dem Brightspace-Konto des Studierenden zugewiesen. Wenn der Studierende seinen Namen oder Avatar in Brightspace ändert, synchronisiert der nächste LTI-Start die Änderung.

#### iframe-Höhe und Größenanpassung

FastComments sendet die PostMessage `org.imsglobal.lti.frameResize` bei jedem Thread-Render und bei Inhaltsänderungen (neuer Kommentar, aufklappbare Antworten). Brightspace hört auf diese Nachricht und passt die iframe-Höhe an, sodass der Thread nicht abgeschnitten wird und kein innerer Scrollbalken angezeigt wird.

Wenn das iframe auf einer festen, zu geringen Höhe bleibt:

- Bestätigen Sie, dass der Kurs über HTTPS geladen wird. Brightspace hört auf PostMessages von Mixed-Content-Frames nicht.
- Stellen Sie sicher, dass keine Browser-Erweiterung den PostMessage-Kanal blockiert.
- Bei Inline-Einbettungen in einem HTML-Thema darf das umgebende HTML das iframe nicht in einen Container mit fester Höhe einbetten. Entfernen Sie jegliches inline `style="height: ..."` vom übergeordneten Element.

#### Brightspace-spezifische Fallstricke

**Tool wird im Add Existing-Picker nicht angezeigt.** Die Bereitstellung ist für die Organisationseinheit dieses Kurses nicht aktiviert. Ein Administrator muss die Organisationseinheit (oder eine übergeordnete) zur Org Units-Liste der Bereitstellung hinzufügen. Die Tool-Registrierung allein reicht nicht aus; die Bereitstellung bestimmt, welche Kurse das Tool sehen.

**`deployment_id`-Mismatch beim Start.** FastComments merkt sich bei Vertrauen-durch-Erstkontakt (TOFU) die erste `deployment_id`, die es für eine Registrierung sieht. Wenn ein Administrator die ursprüngliche Bereitstellung löscht und eine neue erstellt, werden Starts von der neuen Bereitstellung mit einem Deployment-Mismatch-Fehler abgelehnt. Die Lösung besteht darin, FastComments neu zu registrieren (generieren Sie eine neue Registrierungs-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hier abrufen</a>) und führen Sie die Dynamische Registrierung erneut aus); der alte Konfigurationsdatensatz wird ersetzt.

**Tool startet, zeigt aber "Invalid LTI launch".** Der Kurs befindet sich in einer anderen Mandanten-/Organisationsstruktur als die Bereitstellung abdeckt, oder die Bereitstellung wurde nach der Registrierung deaktiviert. Überprüfen Sie **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled**-Schalter und die Org Unit-Liste der Bereitstellung.

**Namen und Rollen fehlen in FastComments.** Brightspace sendet LTI-Starts mit Names and Role Provisioning Services (NRPS)-Claims. Wenn ein Kurs von einem älteren LTI 1.1-Link aktualisiert wurde, fehlen dem Start `name`- und `email`-Claims. Fügen Sie das FastComments-Thema erneut über **Add Existing** hinzu (migrieren Sie den alten Link nicht), damit der Start LTI 1.3 verwendet.

**Einbettung zeigt eine Anmeldeseite statt Auto-SSO.** Das HTML-Thema wurde als einfaches `<iframe>` eingefügt, das auf FastComments zeigt, statt über **Insert Stuff** > **LTI Advantage**. Einfache iframes überspringen den LTI-Start und landen Benutzer auf der öffentlich zugänglichen FastComments-Seite. Löschen Sie das iframe und fügen Sie es erneut über den Insert Stuff-Workflow ein.