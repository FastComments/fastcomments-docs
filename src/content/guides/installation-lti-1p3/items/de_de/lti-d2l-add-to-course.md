Diese Seite behandelt das Hinzufügen von FastComments zu einem Brightspace-Kurs, nachdem ein Administrator das Tool registriert und eine Bereitstellung erstellt hat. Wenn das Tool noch nicht registriert ist, lesen Sie zuerst die D2L-Registrierungsanleitung.

<div class="screenshot white-bg">
    <div class="title">FastComments eingebettet als Einheitsthema in Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments running inside a Brightspace unit, showing threaded comments and an @-mention picker" />
</div>

Brightspace bietet zwei Content-Erstellungsoberflächen: **Classic Content** und die **New Content Experience** (auch **Lessons** genannt). Beide bieten Zugriff auf FastComments, aber die Menüpfade unterscheiden sich. Jeder Abschnitt unten behandelt beide Varianten, wo sie auseinandergehen.

#### Locate the FastComments Tool

Das FastComments-Tool erscheint an zwei Stellen im Kurs-Content-Editor:

1. Dem Aktivitätsauswahlfenster, erreichbar über die **Add Existing**-Schaltfläche eines Moduls/einer Einheit (in älteren Brightspace-Versionen beschriftet als **Add Existing Activities**). In aktuellen Brightspace-Versionen erscheint FastComments direkt im Auswahlfenster; in älteren Versionen ist es unter einem Untermenü **External Learning Tools** verschachtelt. Jeder Pfad fügt FastComments als eigenständiges Thema hinzu.
2. Dem **Insert Stuff**-Dialog im HTML-Editor, unter **LTI Advantage**. Dies bettet FastComments inline in ein HTML-Thema ein über den LTI-Deep-Linking-Flow.

Wenn FastComments in keinem der Auswahlfenster erscheint, ist die Bereitstellung für die Organisations-Einheit, die den Kurs enthält, nicht aktiviert. Bitten Sie Ihren Brightspace-Administrator, **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments** zu öffnen, die Bereitstellung zu öffnen und die Organisations-Einheit des Kurses (oder eine übergeordnete Organisations-Einheit) unter **Org Units** hinzuzufügen.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Öffnen Sie den Kurs und klicken Sie in der Navigationsleiste auf **Content**.
2. Wählen Sie das Modul aus, das die Diskussion enthalten soll (oder erstellen Sie eines über **Add a module**).
3. Klicken Sie auf **Add Existing** (älteres Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. Klicken Sie im Auswahlfenster auf **FastComments**. Brightspace legt ein Thema im Modul an und bringt Sie zur Inhaltsansicht zurück.
5. Klicken Sie das neue Thema an. Benennen Sie es mit dem Inline-Titelfeld um, z. B. `FastComments Discussion`.

New Content Experience (Lessons):

1. Öffnen Sie den Kurs und klicken Sie auf **Content**.
2. Öffnen Sie die Einheit und die Lesson, die die Diskussion enthalten soll.
3. Klicken Sie **Add** > **Existing Activity** und wählen Sie **FastComments** (älteres Brightspace: unter **External Learning Tools** verschachtelt).
4. Die Aktivität wird zur Lesson hinzugefügt.
5. Klicken Sie den Aktivitätstitel an, um ihn umzubenennen.

Beim ersten Öffnen des Themas durch einen Benutzer (Dozent oder Student) initialisiert FastComments den Thread für diesen Resource Link. Der Thread ist an die Resource Link ID gebunden, sodass Umbenennen oder Verschieben des Themas nicht verändert, welcher Thread geladen wird.

#### Embed FastComments Inline in an HTML Topic

Verwenden Sie diesen Ablauf, wenn Kommentare unter einer Lektüre, einem Video oder anderem Inhalt auf derselben Themenseite erscheinen sollen, anstatt als separates Thema.

1. Öffnen oder erstellen Sie ein HTML-Thema im Modul/der Lesson.
2. Klicken Sie auf **Edit HTML**, um den Brightspace-HTML-Editor zu öffnen.
3. Platzieren Sie den Cursor an der Stelle, an der der Kommentar-Thread erscheinen soll.
4. Klicken Sie auf die Schaltfläche **Insert Stuff** (Puzzle-Teil-Symbol in der Editor-Symbolleiste).
5. Scrollen Sie im Insert Stuff-Dialog zu **LTI Advantage** und klicken Sie **FastComments**.
6. FastComments öffnet einen Deep-Linking-Picker. Bestätigen Sie die Platzierung (die Standardoptionen eignen sich für Content-Diskussionen); klicken Sie auf **Insert** oder **Continue**.
7. Brightspace kehrt mit einem Platzhalterblock, der den LTI-Start repräsentiert, zum HTML-Editor zurück. Klicken Sie auf **Save and Close** im Thema.

Wenn das Thema geladen wird, ersetzt Brightspace den Platzhalter durch ein iframe, das FastComments automatisch per LTI startet. Studierende sehen den Diskussions-Thread inline.

Ein einzelnes HTML-Thema kann mehrere per Deep Link eingebettete FastComments-Instanzen enthalten. Jede Einbettung erhält ihren eigenen Thread, da jeder Deep Link eine eigene Resource Link ID erzeugt.

#### Module Topic vs Inline Quicklink

Wählen Sie den Ansatz mit einem **Modulthema**, wenn:

- Die Diskussion die primäre Aktivität für diesen Schritt im Modul ist.
- Sie möchten, dass das Thema im Inhaltsverzeichnis von Brightspace, im Abschluss-Tracking und in Class Progress erscheint.

Wählen Sie die **Inline-Einbettung**, wenn:

- Kommentare unter anderem Inhalt auf derselben Seite stehen sollen.
- Sie keinen separaten, im Inhaltsverzeichnis abschlussverfolgbaren Eintrag wünschen.

#### Visibility, Draft, and Release Conditions

Ein neues FastComments-Thema ist standardmäßig für Studierende sichtbar. Um es während der Einrichtung zu verbergen:

1. Klicken Sie im Content-Editor den Thema-Titel an (Classic) oder das Drei-Punkte-Menü der Aktivität (New Content Experience).
2. Setzen Sie den Status auf **Draft** (Classic) oder schalten Sie die **Visibility** aus (New Content Experience).

Entwurfs-Themen sind für Studierende unsichtbar. Dozierende und TAs sehen sie weiterhin mit einem "Draft"-Badge.

Um das Thema auf eine bestimmte Gruppe oder Sektion zu beschränken:

1. Öffnen Sie das Thema.
2. Klicken Sie im Thema-Titelmenü auf **Edit Properties In-place** (Classic) oder **Edit** > **Restrictions** (New Content Experience).
3. Klicken Sie unter **Release Conditions** auf **Create**.
4. Wählen Sie **Group enrollment** oder **Section enrollment**, wählen Sie die Gruppe/Sektion aus und speichern Sie.

Freigabebedingungen wirken zusammen mit FastComments' eigener Rollen-Zuordnung. Studierende, die das Thema nicht sehen dürfen, erhalten keinen LTI-Start.

#### What Students See on First Launch

Wenn ein Studierender das Thema anklickt (oder ein HTML-Thema mit einer Einbettung lädt):

1. Führt Brightspace den LTI 1.3-Launch im Hintergrund aus.
2. Empfängt FastComments den Namen des Studierenden, die E-Mail, die Avatar-URL und die LMS-Rolle und meldet ihn automatisch an. Es gibt keine FastComments-Anmeldeaufforderung.
3. Der Kommentar-Thread für diesen Resource Link wird innerhalb des Brightspace-iframes gerendert.

Rollen-Zuordnung beim Launch:

- Brightspace `Administrator` wird für den Thread zu einem FastComments **admin** (volle Moderation, Löschen, Sperren und Konfigurationszugriff).
- Brightspace `Instructor` wird zu einem FastComments **moderator** (Anheften, Ausblenden, Löschen, Sperren).
- Alle anderen Rollen (`Learner`, `TeachingAssistant`, etc.) werden zu normalen Kommentierenden.

Kommentare werden dem Brightspace-Konto des Studierenden zugeordnet. Wenn der Studierende seinen Namen oder Avatar in Brightspace ändert, synchronisiert der nächste LTI-Start die Änderung.

#### Iframe Height and Resize

FastComments sendet die `org.imsglobal.lti.frameResize`-postMessage bei jedem Thread-Render und bei Inhaltsänderungen (neuer Kommentar, aufgeklappte Antworten). Brightspace hört auf diese Nachricht und passt die iframe-Höhe an, sodass der Thread nicht abgeschnitten wird und kein innerer Scrollbalken angezeigt wird.

Wenn das iframe auf einer kurzen festen Höhe bleibt:

- Stellen Sie sicher, dass der Kurs über HTTPS geladen wird. Brightspace' postMessage-Listener lehnt Mixed-Content-Frames ab.
- Stellen Sie sicher, dass keine Browsererweiterung den postMessage-Kanal blockiert.
- Bei Inline-Einbettungen in einem HTML-Thema darf das umgebende HTML das iframe nicht in einen Container mit fester Höhe einschließen. Entfernen Sie any inline `style="height: ..."` aus dem Elternelement.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Die Bereitstellung ist für die Organisations-Einheit dieses Kurses nicht aktiviert. Ein Administrator muss die Organisations-Einheit (oder eine übergeordnete) zur Org Units-Liste der Bereitstellung hinzufügen. Die Tool-Registrierung allein reicht nicht aus; die Bereitstellung bestimmt, welche Kurse das Tool sehen.

**`deployment_id` mismatch on launch.** FastComments pinnt beim TOFU-Prinzip die erste `deployment_id`, die es für eine Registrierung sieht. Löscht ein Administrator die ursprüngliche Bereitstellung und erstellt eine neue, werden Starts von der neuen Bereitstellung mit einem Bereitstellungs-Mismatch-Fehler abgelehnt. Die Lösung ist, FastComments neu zu registrieren (eine neue Registrierungs-URL generieren und die dynamische Registrierung erneut durchführen); der alte Konfigurationsdatensatz wird ersetzt.

**Tool launches but shows "Invalid LTI launch".** Der Kurs befindet sich in einer anderen Mandanten-/Organisationsstruktur, als die die Bereitstellung abdeckt, oder die Bereitstellung wurde nach der Registrierung deaktiviert. Überprüfen Sie **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled**-Schalter und die Organisations-Einheiten-Liste der Bereitstellung.

**Names and roles missing inside FastComments.** Brightspace liefert LTI-Starts mit Names and Role Provisioning Services (NRPS)-Claims. Wenn ein Kurs aus einem älteren LTI 1.1-Link migriert wurde, fehlen im Start die `name`- und `email`-Claims. Fügen Sie das FastComments-Thema über **Add Existing** erneut hinzu (migrieren Sie nicht den alten Link), damit der Start LTI 1.3 verwendet.

**Embed shows a login screen instead of auto-SSO.** Das HTML-Thema wurde als einfaches `<iframe>` eingefügt, das auf FastComments zeigt, statt über **Insert Stuff** > **LTI Advantage**. Einfache iframes überspringen den LTI-Start und landen die Nutzer auf der öffentlich zugänglichen FastComments-Seite. Löschen Sie das iframe und fügen Sie es über den Insert Stuff-Flow erneut ein.